use crate::git::utils::create_directory;
use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, instrument};

#[instrument(err)]
pub fn init_git_repo(p: PathBuf) -> Result<String> {
    // first we create the .git directory
    create_directory(p.clone(), ".git/")?;

    // adjust the path for creating directories
    let new_path = {
        let mut x = p.clone();
        x.push(".git/");
        x
    };

    // now the .git/objects directory
    create_directory(new_path.clone(), "objects/")?;

    // now the .git/refs directory
    create_directory(new_path.clone(), "refs/")?;

    // now we'll create the HEAD file
    let mut head_file_path = new_path.clone();
    head_file_path.push("HEAD");
    std::fs::write(head_file_path, "ref: refs/heads/main\n")?;

    debug!("initialized repo");

    Ok(new_path.to_string_lossy().to_string())
}

#[instrument(err)]
pub fn init_git_repo_in_current_dir() -> Result<()> {
    let cur = std::env::current_dir()?;
    debug!("creating git repo in {cur:?}");
    init_git_repo(cur)?;
    Ok(())
}
