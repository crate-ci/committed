use snapbox::str;

#[test]
fn wip() {
    let root = snapbox::dir::DirRoot::mutable_temp().unwrap();
    let root_dir = root.path().unwrap();
    std::fs::write(root_dir.join("committed.toml"), "").unwrap();

    git2::Repository::init(root_dir).unwrap();

    let message = "WIP: bad times ahead";
    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("committed"))
        .arg("--commit-file=-")
        .current_dir(root_dir)
        .stdin(message)
        .assert()
        .code(1)
        .stdout_eq(str![[r#"
-: error Work-in-progress commits must be cleaned up

"#]])
        .stderr_eq(str![]);

    root.close().unwrap();
}
