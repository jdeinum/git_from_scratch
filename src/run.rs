use crate::{
    cli::{Cli, Commands},
    git::{cat_git_obect, init_git_repo_in_current_dir},
};
use anyhow::{Result, ensure};
use clap::Parser;
use tracing::info;

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    // SAFETY:
    // safe as long as clap sees that there is no argument passed into the function
    match &cli.command.unwrap() {
        Commands::Init {} => {
            info!("Initializing a git repo!");
            init_git_repo_in_current_dir()?;
        }
        Commands::CatFile { hash } => {
            info!("generating hash");
            cat_git_obect(&hash)?;
        }
    }

    Ok(())
}
