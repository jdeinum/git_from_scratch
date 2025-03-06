use crate::utils::TRACING;
use anyhow::Result;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;
use temp_testdir::TempDir;

const FILENAME: &str = "test.txt";
const CONTENTS: &str = "hello world!";

#[test]
pub fn test_git_hash() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    todo!()
}
