use crate::utils::{TRACING, create_and_swap_to_temp_dir};
use anyhow::{Context, Result};
use assert_cmd::Command;
use std::{path::PathBuf, sync::LazyLock};

#[test]
pub fn test_git_init() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a temporary directory and switch to that directory
    let temp_dir = create_and_swap_to_temp_dir()
        .context("create temp dir")
        .unwrap();

    // run the git init command
    let output = Command::cargo_bin("git")
        .unwrap()
        .current_dir(&temp_dir.path())
        .arg("init")
        .assert();

    // assert it ran ok
    output.success();

    // check to make sure the directory exists
    // TODO: check all dirs
    let mut p: PathBuf = PathBuf::from(temp_dir.path());
    p.push(".git");
    assert!(p.exists());

    Ok(())
}
