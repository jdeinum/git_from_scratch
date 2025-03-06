use crate::utils::create_tree;
use crate::utils::{TRACING, create_object_and_return_hash};
use anyhow::Result;
use std::process::Command;
use std::sync::LazyLock;
use temp_testdir::TempDir;
use tracing::debug;

const FILENAME: &str = "test.txt";
const CONTENTS: &str = "hello world!";

#[test]
pub fn test_git_write_tree() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    todo!()
}
