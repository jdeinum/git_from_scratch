use anyhow::Result;
use std::path::PathBuf;
use tracing::{debug, info, instrument};

#[instrument(err)]
fn init_git_repo(p: PathBuf) -> Result<()> {
    let new_path = {
        let mut x = p.clone();
        x.push(".git/");
        x
    };
    debug!("creating git repo in {new_path:?}");
    std::fs::create_dir(new_path)?;
    Ok(())
}

#[instrument(err)]
pub fn init_git_repo_in_current_dir() -> Result<()> {
    let cur = std::env::current_dir()?;
    info!("current directory: {}", cur.to_string_lossy());
    init_git_repo(cur)
}
