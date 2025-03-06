use crate::utils::{TRACING, create_and_swap_to_temp_dir};
use anyhow::{Context, Result};
use assert_cmd::output::OutputOkExt;
use std::{path::PathBuf, sync::LazyLock};

#[test]
pub fn test_git_hash() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create and switch to our temp dir
    let temp_dir = create_and_swap_to_temp_dir()
        .context("create and change dir to temp dir")
        .unwrap();

    // create a git directory using the git command
    let _ = std::process::Command::new("git").arg("init").unwrap();

    // create a temporary file in the dir that we can compare
    let mut filename: PathBuf = temp_dir.path().into();
    filename.push("test.txt");
    std::fs::write(&filename, b"hello world!").unwrap();

    // write the object to storage using git hash-object -w
    let hash = assert_cmd::Command::cargo_bin("git")
        .unwrap()
        .arg("hash-object")
        .arg("-w")
        .arg("test.txt")
        .output()
        .unwrap()
        .stdout;
    let hash = std::str::from_utf8(&hash).unwrap();

    // now run our program to cat the file
    let output = std::process::Command::new("git")
        .arg("cat-file")
        .arg("-p")
        .arg(hash.trim())
        .output()
        .unwrap()
        .stdout;

    assert_eq!(output, b"hello world!");

    Ok(())
}
