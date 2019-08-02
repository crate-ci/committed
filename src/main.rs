// 2015-edition macros.
#[macro_use]
extern crate clap;

use std::fs;
use std::io::Read;
use std::io::Write;

use structopt::StructOpt;

mod checks;
mod config;
mod git;
mod report;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    commits: Option<String>,

    #[structopt(long = "commit-file", parse(from_os_str))]
    commit_file: Option<std::path::PathBuf>,

    #[structopt(long = "work-tree", parse(from_os_str), default_value = ".")]
    work_tree: std::path::PathBuf,

    #[structopt(long = "config", parse(from_os_str))]
    config: Option<std::path::PathBuf>,

    #[structopt(
        long = "format",
        raw(possible_values = "&Format::variants()", case_insensitive = "true"),
        default_value = "brief"
    )]
    format: Format,

    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
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

fn load_toml(path: &std::path::Path) -> Result<config::Config, failure::Error> {
    let mut f = fs::File::open(path)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    toml::from_str(&text).map_err(|e| e.into())
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

    let report = options.format.report();

    let mut failed = false;
    if let Some(path) = options.commit_file.as_ref() {
        let mut text = String::new();
        if path == std::path::Path::new("-") {
            std::io::stdin().read_to_string(&mut text)?;
        } else {
            let mut f = fs::File::open(path)?;
            f.read_to_string(&mut text)?;
        }
        failed = checks::check_all(path.as_path().into(), &text, &config, report)?;
    } else if let Some(commits) = options.commits.as_ref() {
        let revspec = git::RevSpec::parse(&repo, commits)?;
        for commit in revspec.iter() {
            let message = commit.message().unwrap();
            failed = failed || checks::check_all(commit.id().into(), message, &config, report)?;
        }
    } else if grep_cli::is_readable_stdin() {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text)?;
        failed = checks::check_all(std::path::Path::new("-").into(), &text, &config, report)?;
    } else {
        debug_assert_eq!(options.commits, None);
        let commits = "HEAD";
        let revspec = git::RevSpec::parse(&repo, commits)?;
        for commit in revspec.iter() {
            let message = commit.message().unwrap();
            failed = failed || checks::check_all(commit.id().into(), message, &config, report)?;
        }
    }

    Ok(if failed { 1 } else { 0 })
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
