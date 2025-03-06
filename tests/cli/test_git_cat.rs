use crate::utils::TRACING;
use anyhow::Result;
use std::sync::LazyLock;

const FILENAME: &str = "test.txt";
const CONTENTS: &str = "hello world!";

#[test]
pub fn test_git_cat() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    todo!()
}
