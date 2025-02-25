use crate::utils::TRACING;
use anyhow::Result;
use assert_cmd::Command;
use std::path::PathBuf;
use std::sync::LazyLock;
use temp_testdir::TempDir;
use tracing::debug;

#[test]
pub fn test_git_cat() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/rstest", false);
    let mut full_path = PathBuf::from(temp.as_ref());
    full_path.push(".git/");

    // initialize the directory
    let mut cmd = Command::cargo_bin("git")?;
    cmd.arg("init");
    cmd.current_dir(&temp.as_ref());
    cmd.assert().success();

    // create a file in the temporary directory
    let mut file_path = PathBuf::from(temp.as_ref());
    file_path.push("test.txt");
    std::fs::write(file_path, "hello world!")?;

    // list our the files in the directory
    let files = std::process::Command::new("ls")
        .current_dir(&temp)
        .output()?
        .stdout;
    let files = String::from_utf8(files)?;
    debug!("files: {files}");

    // now we will get the git hash for that file
    let hash = std::process::Command::new("git")
        .current_dir(&temp)
        .arg("hash-object")
        .arg("-w")
        .arg("test.txt")
        .output()?
        .stdout;

    let hash = String::from_utf8(hash)?;
    debug!("file hash: {hash}");

    // run the command
    let mut cmd = Command::cargo_bin("git")?;
    cmd.arg("cat-file");
    cmd.arg("-p");
    cmd.arg(hash.trim());
    cmd.current_dir(&temp.as_ref());

    // first assert the command ran successfully
    let output = cmd.output()?.stdout;
    let output = String::from_utf8(output)?;

    assert_eq!(output.trim(), "hello world!".to_string());

    Ok(())
}
