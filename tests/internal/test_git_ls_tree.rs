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

    // now we need to add the files to the index so they get included in the tree we make
    debug!("adding files to index");
    let _ = Command::new("git")
        .current_dir(temp.as_ref())
        .arg("add")
        .arg(".")
        .output()?;

    // now create the tree we are interested in
    let hash = {
        let o = Command::new("git")
            .current_dir(temp.as_ref())
            .arg("write-tree")
            .output()?
            .stdout;
        String::from_utf8(o)
    }?;
    debug!("tree hash: {hash:?}");

    // create a buffer we'll be writing into
    let mut buf: Vec<u8> = Vec::new();

    // now cat the file
    ls_tree_git(&hash.trim(), LsTreeOptions { name_only: true }, &mut buf)?;

    // we'll use the output from the actual git tool to ensure our output is as expected
    let git_output = {
        let o = Command::new("git")
            .current_dir(temp.as_ref())
            .arg("ls-tree")
            .arg("--name-only")
            .arg(hash.trim())
            .output()?
            .stdout;
        String::from_utf8(o)
    }?;

    assert_eq!(String::from_utf8(buf)?, git_output);

    Ok(())
}
