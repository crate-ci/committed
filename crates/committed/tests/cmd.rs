use snapbox::str;

#[test]
fn wip() {
    let message = "WIP: bad times ahead";
    let config = "";

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

    snapbox::cmd::Command::new(snapbox::cmd::cargo_bin!("committed"))
        .arg("HEAD")
        .current_dir(root_dir)
        .assert()
        .code(1)
        .stdout_eq(str![[r#"
[..]: error Work-in-progress commits must be cleaned up

"#]])
        .stderr_eq(str![]);

    root.close().unwrap();
}
