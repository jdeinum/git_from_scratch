use crate::utils::{TRACING, create_object_and_return_hash};
use anyhow::Result;
use git_from_scratch::git::cat_git_object;
use git_from_scratch::git::init_git_repo;
use std::path::PathBuf;
use std::sync::LazyLock;
use temp_testdir::TempDir;

#[test]
pub fn test_git_cat() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/rstest", false);

    // initialize the directory
    let _ = init_git_repo(temp.to_path_buf())?;

    // create a file in the temporary directory
    let mut file_path = PathBuf::from(temp.as_ref());
    file_path.push("test.txt");
    std::fs::write(file_path, "hello world!")?;

    // now we will get the git hash for that file
    let hash = create_object_and_return_hash(temp.to_path_buf().to_string_lossy().as_ref())?;

    // set the current directory
    std::env::set_current_dir(&temp)?;

    // now cat the file
    let output = cat_git_object(&hash)?;

    assert_eq!(output.trim(), "hello world!".to_string());

    Ok(())
}
