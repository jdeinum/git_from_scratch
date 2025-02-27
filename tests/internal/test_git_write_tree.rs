use crate::utils::create_tree;
use crate::utils::{TRACING, create_object_and_return_hash};
use anyhow::Result;
use git_from_scratch::git::init_git_repo;
use git_from_scratch::git::ls_tree_git;
use git_from_scratch::git::{LsTreeOptions, write_tree_git};
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

    // create a new temp directory for testing
    let temp = TempDir::new("/tmp/git_ls_tree", false);

    // set the current directory
    std::env::set_current_dir(&temp)?;

    // initialize the directory
    let _ = init_git_repo(temp.to_path_buf())?;

    // create our testing tree
    let _ = create_tree()?;

    // create our buffer we'll be writing into
    let mut buf: Vec<u8> = Vec::new();

    // write our tree
    // we assume that all of the items in the working directory are commited until we implement the
    // index
    let tree_sha = write_tree_git(&mut buf)?;

    // now we'll add all of the files to the git index so that we can compare our sha1 to the sha1
    // from the real git tool

    // now we'll generate the git sha for our tree in the index
    let expected = "todo";

    assert_eq!(tree_sha.trim(), expected.to_string());

    Ok(())
}
