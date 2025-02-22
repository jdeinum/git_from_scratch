use crate::utils::TRACING;
use anyhow::Result;
use assert_cmd::Command;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;
use temp_testdir::TempDir;

#[test]
pub fn test_git_cat() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/rstest", false);
    let mut full_path = PathBuf::from(temp.as_ref());
    full_path.push(".git/");

    // run the command
    let mut cmd = Command::cargo_bin("git")?;
    cmd.arg("cat-file");
    cmd.arg("-p");
    cmd.arg("hello");
    cmd.current_dir(&temp.as_ref());

    // first assert the command ran successfully
    cmd.assert().success();

    // now make sure the new folder exists
    assert!(Path::new(&full_path).exists());

    // now we'll generate a "hello world" file

    Ok(())
}
