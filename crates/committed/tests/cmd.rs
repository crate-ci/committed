use snapbox::str;

#[test]
fn has_message_fails() {
    run_committed("", "")
        .code(1)
        .stdout_eq(str![[r#"
-: error Empty commits are disallowed

"#]])
        .stderr_eq(str![]);
}

#[test]
fn wip_fails() {
    run_committed("wip bad times ahead", "")
        .code(1)
        .stdout_eq(str![[r#"
-: error Work-in-progress commits must be cleaned up

"#]])
        .stderr_eq(str![]);
}

#[test]
fn wip_config_override() {
    run_committed("wip bad times ahead", "no_wip = false")
        .code(1)
        .stdout_eq(str![[r#"
-: error Subject should be capitalized but found `wip`

"#]])
        .stderr_eq(str![]);
}

#[track_caller]
fn run_committed(message: &str, config: &str) -> snapbox::cmd::OutputAssert {
    let root = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let root_dir = root.path().unwrap();
    let config_path = root_dir.join("committed.toml");
    std::fs::write(&config_path, config).unwrap();

    let assert = snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("committed"))
        .arg("--commit-file=-")
        .arg("--config")
        .arg(&config_path)
        .current_dir(root_dir)
        .stdin(message)
        .assert();

    root.close().unwrap();

    assert
}

#[test]
fn in_repo() {
    run_committed_repo("WIP: bad times ahead", "")
        .code(1)
        .stdout_eq(str![[r#"
[..]: error Work-in-progress commits must be cleaned up

"#]])
        .stderr_eq(str![]);
}

fn run_committed_repo(message: &str, config: &str) -> snapbox::cmd::OutputAssert {
    let root = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let root_dir = root.path().unwrap();
    let config_filename = "committed.toml";
    let config_path = root_dir.join(config_filename);
    std::fs::write(&config_path, config).unwrap();

    let repo = git2::Repository::init(root_dir).unwrap();
    let mut index = repo.index().unwrap();
    index
        .add_path(std::path::Path::new(config_filename))
        .unwrap();
    index.write().unwrap();
    let id = index.write_tree().unwrap();
    let tree = repo.find_tree(id).unwrap();
    let sig = repo.signature().unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])
        .unwrap();

    let assert = snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("committed"))
        .arg("HEAD")
        .current_dir(root_dir)
        .assert();

    root.close().unwrap();

    assert
}
