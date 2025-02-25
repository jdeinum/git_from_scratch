use crate::utils::TRACING;
use anyhow::Result;
use git_from_scratch::git::StoreHash;
use git_from_scratch::git::hash_git_object;
use git_from_scratch::git::init_git_repo;
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

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/rstest", true);

    // initialize the directory
    let _ = init_git_repo(temp.to_path_buf())?;

    // change to the temp
    std::env::set_current_dir(&temp)?;

    // create a file in the temporary directory
    std::fs::write(FILENAME, CONTENTS)?;

    // now we will get the git hash for that file
    let hash = hash_git_object(Path::new(FILENAME), StoreHash::Yes)?;

    // now we'll read the contents of file using the git cat-file command
    let contents = {
        let raw = Command::new("git")
            .current_dir(temp)
            .arg("cat-file")
            .arg("-p")
            .arg(hash)
            .output()?
            .stdout;
        String::from_utf8(raw)
    }?;

    assert_eq!(contents.trim(), CONTENTS);

    Ok(())
}
