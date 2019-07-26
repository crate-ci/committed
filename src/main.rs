#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Options {}

fn run() -> Result<i32, failure::Error> {
    let options = Options::from_args().infer();

    Ok(0)
}

fn main() {
    let code = run().unwrap();
    std::process::exit(code);
}
