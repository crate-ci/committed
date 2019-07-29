use std::fs;
use std::io::Read;

use structopt::StructOpt;

mod config;
mod git;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    commits: String,

    #[structopt(parse(from_os_str), default_value = ".")]
    work_tree: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    config: Option<std::path::PathBuf>,
}

fn load_toml(path: &std::path::Path) -> Result<config::Config, failure::Error> {
    let mut f = fs::File::open(path)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    toml::from_str(&text).map_err(|e| e.into())
}

fn run() -> Result<i32, failure::Error> {
    let options = Options::from_args();

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

    let revspec = git::RevSpec::parse(&repo, &options.commits)?;
    for commit in revspec.iter() {
        if config.style() == config::Style::Conventional {
            committed::conventional::Message::parse(commit.message().unwrap()).unwrap();
        }
    }

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
