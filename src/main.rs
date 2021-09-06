#![allow(clippy::unnecessary_wraps)]

// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::fs;
use std::io::Read;
use std::io::Write;

use proc_exit::WithCodeResultExt;
use structopt::StructOpt;

mod checks;
mod config;
mod git;
mod report;

#[derive(Debug, StructOpt)]
#[structopt(
    setting = structopt::clap::AppSettings::UnifiedHelpMessage,
    setting = structopt::clap::AppSettings::DeriveDisplayOrder,
    setting = structopt::clap::AppSettings::DontCollapseArgsInUsage
)]
#[structopt(group = structopt::clap::ArgGroup::with_name("mode").multiple(false))]
struct Options {
    #[structopt(group = "mode")]
    commits: Option<String>,

    #[structopt(long, parse(from_os_str), group = "mode")]
    /// Check a message in a file with `-` for stdin
    commit_file: Option<std::path::PathBuf>,

    #[structopt(long, parse(from_os_str), default_value = ".")]
    work_tree: std::path::PathBuf,

    #[structopt(long, parse(from_os_str))]
    config: Option<std::path::PathBuf>,

    #[structopt(long, group = "mode")]
    /// Write the current configuration to file with `-` for stdout
    dump_config: Option<std::path::PathBuf>,

    #[structopt(long, overrides_with("merge-commit"))]
    no_merge_commit: bool,
    #[structopt(long, overrides_with("no-merge-commit"), hidden(true))]
    merge_commit: bool,

    #[structopt(
        long = "format",
        possible_values(&Format::variants()),
        case_insensitive(true),
        default_value("brief")
    )]
    format: Format,

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Options {
    fn merge_commit(&self) -> Option<bool> {
        match (self.no_merge_commit, self.merge_commit) {
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None,
            (true, true) => unreachable!("Structopt should make this impossible"),
        }
    }
}

arg_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    enum Format {
        Silent,
        Brief,
        Json,
    }
}

impl Format {
    fn report(self) -> report::Report {
        match self {
            Format::Silent => report::print_silent,
            Format::Brief => report::print_brief,
            Format::Json => report::print_json,
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Brief
    }
}

fn load_toml(path: &std::path::Path) -> Result<config::Config, anyhow::Error> {
    let mut f = fs::File::open(path)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    toml::from_str(&text).map_err(|e| e.into())
}

pub fn init_logging(level: Option<log::Level>) {
    if let Some(level) = level {
        let mut builder = env_logger::Builder::new();

        builder.filter(None, level.to_level_filter());

        if level == log::LevelFilter::Trace {
            builder.format_timestamp_secs();
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

        builder.init();
    }
}

fn run() -> proc_exit::ExitResult {
    let options = Options::from_args();

    init_logging(options.verbose.log_level());

    let repo = options
        .work_tree
        .canonicalize()
        .with_code(proc_exit::Code::USAGE_ERR)?;

    let repo = git2::Repository::discover(repo).with_code(proc_exit::Code::USAGE_ERR)?;
    let mut config = if let Some(config_path) = options.config.as_ref() {
        load_toml(config_path).with_code(proc_exit::Code::CONFIG_ERR)?
    } else {
        let config_path = repo
            .workdir()
            .ok_or_else(|| anyhow::anyhow!("Cannot work on bare repo"))
            .with_code(proc_exit::Code::USAGE_ERR)?
            .join("committed.toml");
        if config_path.is_file() {
            load_toml(&config_path).with_code(proc_exit::Code::CONFIG_ERR)?
        } else {
            config::Config::default()
        }
    };
    config.update_merge_commit(options.merge_commit());
    let config = config;

    let report = if options.verbose.is_silent() {
        report::print_silent
    } else {
        options.format.report()
    };

    let ignore_author_re = config
        .ignore_author_re()
        .map(|re| regex::Regex::new(re))
        .transpose()
        .with_code(proc_exit::Code::CONFIG_ERR)?;
    let ignore_commit = |commit: &git2::Commit| {
        let author = commit.author().to_string();
        if let Some(re) = ignore_author_re.as_ref() {
            if re.is_match(&author) {
                return true;
            }
        }
        false
    };

    let mut failed = false;
    if let Some(output_path) = options.dump_config.as_ref() {
        let mut defaulted_config = config::Config::from_defaults();
        defaulted_config.update(config);

        let output =
            toml::to_string_pretty(&defaulted_config).with_code(proc_exit::Code::FAILURE)?;
        if output_path == std::path::Path::new("-") {
            std::io::stdout().write_all(output.as_bytes())?;
        } else {
            std::fs::write(output_path, &output)?;
        }
    } else if let Some(path) = options.commit_file.as_ref() {
        let mut text = String::new();
        if path == std::path::Path::new("-") {
            std::io::stdin().read_to_string(&mut text)?;
        } else {
            let mut f = fs::File::open(path)?;
            f.read_to_string(&mut text)?;
        }
        failed |= checks::check_message(path.as_path().into(), &text, &config, report)
            .with_code(proc_exit::Code::UNKNOWN)?;
    } else if let Some(commits) = options.commits.as_ref() {
        let revspec = git::RevSpec::parse(&repo, commits).with_code(proc_exit::Code::USAGE_ERR)?;
        for commit in revspec.iter() {
            if ignore_commit(&commit) {
                log::trace!("Ignoring {}", commit.id());
            } else {
                log::trace!("Processing {}", commit.id());
                let message = commit.message().unwrap();
                failed |= checks::check_message(commit.id().into(), message, &config, report)
                    .with_code(proc_exit::Code::UNKNOWN)?;
                if !config.merge_commit() {
                    failed |= checks::check_merge_commit(commit.id().into(), &commit, report)
                        .with_code(proc_exit::Code::UNKNOWN)?;
                }
            }
        }
    } else if grep_cli::is_readable_stdin() {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text)?;
        failed |= checks::check_message(std::path::Path::new("-").into(), &text, &config, report)
            .with_code(proc_exit::Code::UNKNOWN)?;
    } else {
        debug_assert_eq!(options.commits, None);
        let commit = repo
            .head()
            .with_code(proc_exit::Code::USAGE_ERR)?
            .peel_to_commit()
            .with_code(proc_exit::Code::USAGE_ERR)?;
        if ignore_commit(&commit) {
            log::trace!("Ignoring {}", commit.id());
        } else {
            log::trace!("Processing {}", commit.id());
            let message = commit.message().unwrap();
            failed |= checks::check_message(commit.id().into(), message, &config, report)
                .with_code(proc_exit::Code::UNKNOWN)?;
            if !config.merge_commit() {
                failed |= checks::check_merge_commit(commit.id().into(), &commit, report)
                    .with_code(proc_exit::Code::UNKNOWN)?;
            }
        }
    }

    if failed {
        proc_exit::Code::FAILURE.ok()
    } else {
        proc_exit::Code::SUCCESS.ok()
    }
}

fn main() {
    let result = run();
    proc_exit::exit(result);
}
