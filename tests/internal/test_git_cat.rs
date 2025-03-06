use crate::utils::{TRACING, create_object_and_return_hash};
use anyhow::Result;
use git_from_scratch::git::init_git_repo;
use git_from_scratch::git::run;
use std::sync::LazyLock;
use temp_testdir::TempDir;

const FILENAME: &str = "test.txt";
const CONTENTS: &str = "hello world!";

#[test]
pub fn test_git_cat() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/git_cat", false);

    // set the current directory
    std::env::set_current_dir(&temp)?;

    // initialize the directory
    let _ = init_git_repo(temp.to_path_buf())?;

    // create a file in the temporary directory
    std::fs::write(FILENAME, CONTENTS)?;

    // now we will get the git hash for that file
    let hash =
        create_object_and_return_hash(temp.to_path_buf().to_string_lossy().as_ref(), FILENAME)?;

    // create a buffer we'll be writing into
    let mut buf: Vec<u8> = Vec::new();

    // now cat the file
    run(&hash, &mut buf)?;

    assert_eq!(buf, CONTENTS.as_bytes());

    Ok(())
}
