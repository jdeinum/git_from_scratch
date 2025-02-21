use anyhow::Result;
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
