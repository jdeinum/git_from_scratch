use crate::{
    cli::{Cli, Commands},
    git::init_git_repo_in_current_dir,
};
use anyhow::Result;
use clap::Parser;
use tracing::info;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {}) => {
            info!("Initializing a git repo!");
            init_git_repo_in_current_dir()?;
        }
        None => {}
    }

    Ok(())
}
