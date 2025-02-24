use crate::git::{convert_file, git_objects_dir_exists, read_file};
use anyhow::{Result, ensure};
use std::path::PathBuf;
use tracing::{debug, instrument};

#[instrument(err)]
fn init_git_repo(p: PathBuf) -> Result<()> {
    let new_path = {
        let mut x = p.clone();
        x.push(".git/");
        x
    };
    std::fs::create_dir(new_path)?;
    Ok(())
}

#[instrument(err)]
pub fn init_git_repo_in_current_dir() -> Result<()> {
    let cur = std::env::current_dir()?;
    debug!("creating git repo in {cur:?}");
    init_git_repo(cur)
}

// todo: pass in an output object instead of println
pub fn cat_git_object(hash: &str) -> Result<()> {
    ensure!(git_objects_dir_exists(), ".git/objects/ does not exist");

    // read the file
    let buf = read_file(hash)?;

    // convert the file into UTF8
    let content = convert_file(buf)?;

    // print the contents
    print!("{}", content);

    Ok(())
}
