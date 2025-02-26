use crate::utils::{TRACING, create_object_and_return_hash};
use anyhow::Result;
use git_from_scratch::git::LsTreeOptions;
use git_from_scratch::git::init_git_repo;
use git_from_scratch::git::ls_tree_git;
use std::process::Command;
use std::sync::LazyLock;
use temp_testdir::TempDir;
use tracing::debug;

const FILENAME: &str = "test.txt";
const CONTENTS: &str = "hello world!";

#[test]
pub fn test_git_ls_tree() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/git_ls_tree", false);

    // set the current directory
    std::env::set_current_dir(&temp)?;

    // initialize the directory
    let _ = init_git_repo(temp.to_path_buf())?;

    // create a file in the temporary directory
    std::fs::write(FILENAME, CONTENTS)?;

    // create a new directory in the temporary directory
    std::fs::create_dir("foo")?;

    // now we will create our tree structure
    let _ = create_object_and_return_hash(temp.to_path_buf().to_string_lossy().as_ref(), FILENAME)?;
    let _ = create_object_and_return_hash(temp.to_path_buf().to_string_lossy().as_ref(), "foo/")?;

    // create a tree object, and print out the contents using the --name-only flag
    // TODO!
    let output = {
        let o = Command::new("git")
            .current_dir(temp.as_ref())
            .arg("write-tree")
            .output()?
            .stdout;
        String::from_utf8(o)
    }?;

    debug!("write-tree output: {output}");

    // create a buffer we'll be writing into
    let mut buf: Vec<u8> = Vec::new();

    // now cat the file
    ls_tree_git(&output, LsTreeOptions { name_only: true }, &mut buf)?;

    assert_eq!(buf, output.as_bytes());

    Ok(())
}
