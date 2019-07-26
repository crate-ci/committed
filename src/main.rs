use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {
    commits: String,
    #[structopt(parse(from_os_str), default_value = ".")]
    work_tree: std::path::PathBuf,
}

fn run() -> Result<i32, failure::Error> {
    let options = Options::from_args();

    let repo = options.work_tree.canonicalize()?;

    let repo = git2::Repository::discover(repo)?;
    let commits = repo.revparse(&options.commits)?;
    let from = commits.from().unwrap().as_commit().unwrap();
    let to = commits
        .to()
        .map(|o| o.as_commit().unwrap().clone())
        .unwrap_or_else(|| {
            let id = repo.refname_to_id("HEAD").unwrap();
            repo.find_commit(id).unwrap()
        });
    let is_descendant = repo.graph_descendant_of(to.id(), from.id())?;
    if !is_descendant {
        failure::bail!("revspec {} are on separate branches", options.commits)
    }
    if to.id() != from.id() {
        committed::conventional::Message::parse(to.message().unwrap()).unwrap();
        for commit in to.parents().take_while(|c| c.id() != from.id()) {
            committed::conventional::Message::parse(commit.message().unwrap()).unwrap();
        }
    }

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
