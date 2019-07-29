use std::fs;
use std::io::Read;

use structopt::StructOpt;

mod config;

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

struct RevSpec<'r> {
    from: git2::Commit<'r>,
    to: git2::Commit<'r>,
}

impl<'r> RevSpec<'r> {
    fn parse(repo: &'r git2::Repository, revspec: &str) -> Result<Self, failure::Error> {
        let commits = repo.revparse(revspec)?;
        let from = commits.from().unwrap().as_commit().unwrap().clone();
        let to = commits
            .to()
            .map(|o| o.as_commit().unwrap().clone())
            .unwrap_or_else(|| {
                let id = repo.refname_to_id("HEAD").unwrap();
                repo.find_commit(id).unwrap()
            });
        let is_descendant = to.id() == from.id() || repo.graph_descendant_of(to.id(), from.id())?;
        if !is_descendant {
            failure::bail!("revspec {} are on separate branches", revspec)
        }
        Ok(Self { from, to })
    }

    fn iter(&self) -> RevSpecIterator {
        RevSpecIterator {
            revspec: &self,
            needs_to: true,
            needs_parents: true,
            parents: self.to.parents(),
        }
    }
}

struct RevSpecIterator<'r> {
    revspec: &'r RevSpec<'r>,
    needs_to: bool,
    needs_parents: bool,
    parents: git2::Parents<'r, 'r>,
}

impl<'r> Iterator for RevSpecIterator<'r> {
    type Item = git2::Commit<'r>;

    fn next(&mut self) -> Option<git2::Commit<'r>> {
        if self.needs_to {
            if self.revspec.to.id() == self.revspec.from.id() {
                return None;
            } else {
                self.needs_to = false;
                return Some(self.revspec.to.clone());
            }
        } else if self.needs_parents {
            if let Some(parent) = self.parents.next() {
                if parent.id() == self.revspec.from.id() {
                    self.needs_parents = false;
                } else {
                    return Some(parent);
                }
            }
        }
        None
    }
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

    let revspec = RevSpec::parse(&repo, &options.commits)?;
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
