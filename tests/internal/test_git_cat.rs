use crate::utils::{TRACING, create_object_and_return_hash};
use anyhow::Result;
use std::sync::LazyLock;
use temp_testdir::TempDir;

const FILENAME: &str = "test.txt";
const CONTENTS: &str = "hello world!";

#[test]
pub fn test_git_cat() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    todo!()
}
