use std::fs;
use std::io::Read;
use std::io::Write;

use structopt::StructOpt;

mod config;
mod git;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    commits: Option<String>,

    #[structopt(parse(from_os_str), default_value = ".")]
    work_tree: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    config: Option<std::path::PathBuf>,

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn load_toml(path: &std::path::Path) -> Result<config::Config, failure::Error> {
    let mut f = fs::File::open(path)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    toml::from_str(&text).map_err(|e| e.into())
}

fn check_subject_length(message: &str, max_length: usize) -> Result<(), failure::Error> {
    let subject = message
        .split('\n')
        .next()
        .ok_or_else(|| failure::Context::new("Commit cannot be empty"))?;
    let subject = subject.trim_end();
    let count = unicode_segmentation::UnicodeSegmentation::graphemes(subject, true).count();
    if max_length < count {
        failure::bail!(
            "Commit subject is {}, exceeding the max length of {}",
            count,
            max_length
        );
    }
    Ok(())
}

fn check_line_length(message: &str, max_length: usize) -> Result<(), failure::Error> {
    for line in message.split('\n') {
        let line = line.trim_end();
        let count = unicode_segmentation::UnicodeSegmentation::graphemes(line, true).count();
        if max_length < count {
            failure::bail!(
                "Commit line is {}, exceeding the max length of {}",
                count,
                max_length
            );
        }
    }
    Ok(())
}

fn check_capitalized_subject(subject: &str) -> Result<(), failure::Error> {
    let first = subject
        .chars()
        .next()
        .ok_or_else(|| failure::Context::new("Subject cannot be empty"))?;
    if !first.is_uppercase() {
        failure::bail!("Subject must be capitalized: `{}`", subject);
    }
    Ok(())
}

fn check_subject_not_punctuated(subject: &str) -> Result<(), failure::Error> {
    let last = subject
        .chars()
        .last()
        .ok_or_else(|| failure::Context::new("Subject cannot be empty"))?;
    if last.is_ascii_punctuation() {
        failure::bail!("Subject must not be punctuated: `{}`", last);
    }
    Ok(())
}

static WIP_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^(wip|WIP)\\b").unwrap());

fn check_wip(message: &str) -> Result<(), failure::Error> {
    if WIP_RE.is_match(message) {
        failure::bail!("Work-in-progress commits must be cleaned up");
    }
    Ok(())
}

static FIXUP_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^fixup! ").unwrap());

fn check_fixup(message: &str) -> Result<(), failure::Error> {
    if FIXUP_RE.is_match(message) {
        failure::bail!("Fixup commits must be squashed");
    }
    Ok(())
}

pub fn get_logging(level: log::Level) -> env_logger::Builder {
    let mut builder = env_logger::Builder::new();

    builder.filter(None, level.to_level_filter());

    if level == log::LevelFilter::Trace {
        builder.default_format_timestamp(false);
    } else {
        builder.format(|f, record| {
            writeln!(
                f,
                "[{}] {}",
                record.level().to_string().to_lowercase(),
                record.args()
            )
        });
    }

    builder
}

fn run() -> Result<i32, failure::Error> {
    let options = Options::from_args();

    let mut builder = get_logging(options.verbose.log_level());
    builder.init();

    let repo = options.work_tree.canonicalize()?;

    let repo = git2::Repository::discover(repo)?;
    let config = if let Some(config_path) = options.config.as_ref() {
        load_toml(config_path)?
    } else {
        let config_path = repo
            .workdir()
            .ok_or_else(|| failure::Context::new("Cannot work on bare repo"))?
            .join("committed.toml");
        if config_path.is_file() {
            load_toml(&config_path)?
        } else {
            config::Config::default()
        }
    };

    let commits = options
        .commits
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("HEAD");
    let revspec = git::RevSpec::parse(&repo, commits)?;
    let no_wip = config.no_wip();
    let no_fixup = config.no_fixup();
    let style = config.style();
    let subject_length = config.subject_length();
    let line_length = config.line_length();
    let subject_capitalized = config.subject_capitalized();
    let subject_not_punctuated = config.subject_not_punctuated();
    for commit in revspec.iter() {
        let message = commit.message().unwrap();
        if !no_wip {
            check_wip(message)?;
        }
        if !no_fixup {
            check_fixup(message)?;
        }
        match style {
            config::Style::Conventional => {
                let parsed = committed::conventional::Message::parse(message).unwrap();
                if subject_capitalized {
                    check_capitalized_subject(parsed.description)?;
                }
                if subject_not_punctuated {
                    check_subject_not_punctuated(parsed.description)?;
                }
            }
            config::Style::None => {
                let parsed = committed::no_style::Message::parse(message).unwrap();
                if subject_capitalized {
                    check_capitalized_subject(parsed.subject)?;
                }
                if subject_not_punctuated {
                    check_subject_not_punctuated(parsed.subject)?;
                }
            }
        }
        if subject_length != 0 {
            check_subject_length(message, subject_length)?;
        }
        if line_length != 0 {
            check_line_length(message, line_length)?;
        }
    }

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
