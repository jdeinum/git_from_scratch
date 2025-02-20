use crate::{
    cli::{Cli, Commands},
    git::init_git_repo_in_current_dir,
};
use anyhow::Result;
use clap::Parser;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init {}) => {
            init_git_repo_in_current_dir()?;
        }
        None => {}
    }

    Ok(())
}
