use crate::utils::TRACING;
use anyhow::Result;
use git_from_scratch::git::init_git_repo;
use std::sync::LazyLock;
use temp_testdir::TempDir;

#[test]
pub fn test_git_init() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/rstest", true);

    // initialize the directory
    let _ = init_git_repo(temp.to_path_buf())?;

    Ok(())
}
