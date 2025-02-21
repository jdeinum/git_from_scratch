use crate::utils::TRACING;
use crate::utils::create_tmpdir_and_chenv;
use anyhow::Result;
use assert_cmd::Command;
use std::path::Path;
use std::sync::LazyLock;
use tracing::debug;
use tracing::info;

#[test]
pub fn test_git_init() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a new temp directory and change our working directory to it
    let path = create_tmpdir_and_chenv()?;
    let full_path = format!("{}/{}", path, ".git/");
    debug!("checking for path {full_path}");

    // run the command
    let mut cmd = Command::cargo_bin("git")?;
    cmd.arg("init");

    // first assert the command ran successfully
    cmd.assert().success();
    info!("output: {}", String::from_utf8(cmd.output()?.stdout)?);

    // now make sure the new folder exists
    assert!(Path::new(&full_path).exists());

    Ok(())
}
