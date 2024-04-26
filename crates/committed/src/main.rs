#![allow(clippy::unnecessary_wraps)]

use std::io::Read;
use std::io::Write;

use clap::Parser;
use proc_exit::prelude::*;

mod checks;
mod color;
mod config;
mod git;
mod report;

const UNKNOWN_ERR: proc_exit::Code = proc_exit::Code::new(2);

#[derive(Debug, Parser)]
#[command(about, version)]
#[command(group = clap::ArgGroup::new("mode").multiple(false))]
struct Options {
    #[arg(group = "mode")]
    commits: Option<String>,

    #[arg(long, group = "mode")]
    /// Check a message in a file with `-` for stdin
    commit_file: Option<std::path::PathBuf>,

    #[arg(long, default_value = ".")]
    work_tree: std::path::PathBuf,

    #[arg(long)]
    config: Option<std::path::PathBuf>,

    #[arg(long, group = "mode")]
    /// Write the current configuration to file with `-` for stdout
    dump_config: Option<std::path::PathBuf>,

    #[arg(long, overrides_with("merge_commit"))]
    no_merge_commit: bool,
    #[arg(long, overrides_with("no_merge_commit"), hide(true))]
    merge_commit: bool,

    #[arg(long, overrides_with("wip"))]
    no_wip: bool,
    #[arg(long, overrides_with("no_wip"), hide(true))]
    wip: bool,

    #[arg(long, overrides_with("fixup"))]
    no_fixup: bool,
    #[arg(long, overrides_with("no_fixup"), hide(true))]
    fixup: bool,

    #[arg(
        long = "format",
        value_enum,
        ignore_case(true),
        default_value = "brief"
    )]
    format: Format,

    #[command(flatten)]
    color: colorchoice_clap::Color,

    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

impl Options {
    fn to_config(&self) -> config::Config {
        config::Config {
            merge_commit: self.merge_commit(),
            no_wip: self.wip().map(|b| !b),
            no_fixup: self.fixup().map(|b| !b),
            ..Default::default()
        }
    }

    fn merge_commit(&self) -> Option<bool> {
        resolve_bool_arg(self.merge_commit, self.no_merge_commit)
    }

    fn wip(&self) -> Option<bool> {
        resolve_bool_arg(self.wip, self.no_wip)
    }

    fn fixup(&self) -> Option<bool> {
        resolve_bool_arg(self.fixup, self.no_fixup)
    }
}

fn resolve_bool_arg(yes: bool, no: bool) -> Option<bool> {
    match (yes, no) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        (false, false) => None,
        (_, _) => unreachable!("StructOpt should make this impossible"),
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum, Default)]
enum Format {
    Silent,
    #[default]
    Brief,
    Json,
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

fn load_toml(path: &std::path::Path) -> Result<config::Config, anyhow::Error> {
    let text = std::fs::read_to_string(path)?;
    toml::from_str(&text).map_err(|e| e.into())
}

fn init_logging(level: Option<log::Level>) {
    if let Some(level) = level {
        let mut builder = env_logger::Builder::new();

        let choice = anstream::AutoStream::choice(&std::io::stderr());
        builder.write_style(if matches!(choice, anstream::ColorChoice::Never) {
            env_logger::WriteStyle::Never
        } else {
            env_logger::WriteStyle::Always
        });

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
    let options = Options::parse();

    options.color.write_global();

    init_logging(options.verbose.log_level());

    let repo = options
        .work_tree
        .canonicalize()
        .with_code(proc_exit::sysexits::USAGE_ERR)?;

    let repo = git2::Repository::discover(repo).with_code(proc_exit::sysexits::USAGE_ERR)?;
    let mut config = if let Some(config_path) = options.config.as_ref() {
        load_toml(config_path).with_code(proc_exit::sysexits::CONFIG_ERR)?
    } else {
        let config_path = repo
            .workdir()
            .ok_or_else(|| anyhow::anyhow!("Cannot work on bare repo"))
            .with_code(proc_exit::sysexits::USAGE_ERR)?
            .join("committed.toml");
        if config_path.is_file() {
            load_toml(&config_path).with_code(proc_exit::sysexits::CONFIG_ERR)?
        } else {
            config::Config::default()
        }
    };
    config.update(options.to_config());
    let config = config;

    let report = if options.verbose.is_silent() {
        report::print_silent
    } else {
        options.format.report()
    };

    let ignore_author_re = config
        .ignore_author_re()
        .map(regex::Regex::new)
        .transpose()
        .with_code(proc_exit::sysexits::CONFIG_ERR)?;
    let ignore_commit = |commit: &git2::Commit<'_>| {
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
            std::io::stdout()
                .write_all(output.as_bytes())
                .to_sysexits()?;
        } else {
            std::fs::write(output_path, &output).to_sysexits()?;
        }
    } else if let Some(path) = options.commit_file.as_ref() {
        let text = if path == std::path::Path::new("-") {
            let mut text = String::new();
            std::io::stdin().read_to_string(&mut text).to_sysexits()?;
            text
        } else {
            std::fs::read_to_string(path).to_sysexits()?
        };
        let text = trim_commit_file(&text);
        failed |= checks::check_message(path.as_path().into(), text, &config, report)
            .with_code(UNKNOWN_ERR)?;
    } else if let Some(commits) = options.commits.as_ref() {
        let revspec =
            git::RevSpec::parse(&repo, commits).with_code(proc_exit::sysexits::USAGE_ERR)?;
        for commit in revspec.iter() {
            let abbrev_id = commit.as_object().short_id().ok();
            let source = abbrev_id
                .as_ref()
                .and_then(|id| id.as_str())
                .map(report::Source::from)
                .unwrap_or_else(|| commit.id().into());
            if ignore_commit(&commit) {
                log::trace!("Ignoring {}", source);
            } else {
                log::trace!("Processing {}", source);
                let message = commit.message().unwrap();
                failed |= checks::check_message(source, message, &config, report)
                    .with_code(UNKNOWN_ERR)?;
                if !config.merge_commit() {
                    failed |= checks::check_merge_commit(source, &commit, report)
                        .with_code(UNKNOWN_ERR)?;
                }
            }
        }
    } else if grep_cli::is_readable_stdin() {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text).to_sysexits()?;
        let text = trim_commit_file(&text);
        failed |= checks::check_message(std::path::Path::new("-").into(), text, &config, report)
            .with_code(UNKNOWN_ERR)?;
    } else {
        debug_assert_eq!(options.commits, None);
        let commit = repo
            .head()
            .with_code(proc_exit::sysexits::USAGE_ERR)?
            .peel_to_commit()
            .with_code(proc_exit::sysexits::USAGE_ERR)?;
        let abbrev_id = commit.as_object().short_id().ok();
        let source = abbrev_id
            .as_ref()
            .and_then(|id| id.as_str())
            .map(report::Source::from)
            .unwrap_or_else(|| commit.id().into());
        if ignore_commit(&commit) {
            log::trace!("Ignoring {}", source);
        } else {
            log::trace!("Processing {}", source);
            let message = commit.message().unwrap();
            failed |=
                checks::check_message(source, message, &config, report).with_code(UNKNOWN_ERR)?;
            if !config.merge_commit() {
                failed |=
                    checks::check_merge_commit(source, &commit, report).with_code(UNKNOWN_ERR)?;
            }
        }
    }

    if failed {
        proc_exit::Code::FAILURE.ok()
    } else {
        proc_exit::Code::SUCCESS.ok()
    }
}

fn trim_commit_file(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        return "";
    }

    let all_comment_re = regex::RegexBuilder::new(r"^(#[^\n]*\n*)+$")
        .dot_matches_new_line(true)
        .build()
        .expect("test ensured regex compiles");
    if all_comment_re.is_match(message) {
        return "";
    }

    let message =
        if let Some(idx) = message.find("# ------------------------ >8 ------------------------") {
            message[..idx].trim()
        } else {
            message
        };

    let trailing_comment_re = regex::RegexBuilder::new(r"^(.*?)(\n+#[^\n]*)*$")
        .dot_matches_new_line(true)
        .build()
        .expect("test ensured regex compiles");
    let captures = trailing_comment_re.captures(message).unwrap();
    captures.get(1).unwrap().as_str()
}

fn main() {
    human_panic::setup_panic!();
    let result = run();
    proc_exit::exit(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::CommandFactory;
        Options::command().debug_assert();
    }

    #[test]
    fn empty() {
        let input = "";
        let expected = "";
        let actual = trim_commit_file(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn user_message() {
        let input = "feat: Hello

Let's do it!

Fixes #10";
        let expected = input;
        let actual = trim_commit_file(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn default_message() {
        let input = "


# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch master
# Your branch is up to date with 'origin/master'.
#
# Changes to be committed:
#	modified:   custom-file.el
#	modified:   init.el
#
# Untracked files:
#	lisp/ob-maven.el
#	url/
#
";
        let expected = "";
        let actual = trim_commit_file(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn user_with_default_message() {
        let input = "feat: Hello

Let's do it!

Fixes #10

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch master
# Your branch is up to date with 'origin/master'.
#
# Changes to be committed:
#	modified:   custom-file.el
#	modified:   init.el
#
# Untracked files:
#	lisp/ob-maven.el
#	url/
#
";
        let expected = "feat: Hello

Let's do it!

Fixes #10";
        let actual = trim_commit_file(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn user_with_verbose_commit() {
        let input = "docs: Add Code of Conduct

Fixes #10

# Please enter the commit message for your changes. Lines starting
# with '#' will be ignored, and an empty message aborts the commit.
#
# On branch chore/repository-setup
# Changes to be committed:
# new file:   docs/CODE_OF_CONDUCT.md
#
# ------------------------ >8 ------------------------
# Do not modify or remove the line above.
# Everything below it will be ignored.
diff --git a/docs/CODE_OF_CONDUCT.md b/docs/CODE_OF_CONDUCT.md
new file mode 100644
index 0000000000000000..a366d6b2f3755024
--- /dev/null
+++ b/docs/CODE_OF_CONDUCT.md
@@ -0,0 +1,134 @@
+# Contributor Covenant Code of Conduct
+
+## Our Pledge
+
+We as members, contributors, and leaders pledge to make participation in our
+community a harassment-free experience for everyone, regardless of age, body
+size, visible or invisible disability, ethnicity, sex characteristics, gender
+identity and expression, level of experience, education, socio-economic status,
+nationality, personal appearance, race, caste, color, religion, or sexual
+identity and orientation.
+
+ ...
";
        let expected = "docs: Add Code of Conduct

Fixes #10";
        let actual = trim_commit_file(input);
        assert_eq!(actual, expected);
    }
}
