use crate::{
    cli::{Cli, Commands},
    git::{cat_git_object, init_git_repo_in_current_dir, is_current_git_directory},
};
use anyhow::{Result, ensure};
use clap::Parser;
use tracing::info;

pub fn run() -> Result<()> {
    // parse will error if no sub command is passed in
    let cli = Cli::parse();

    // if we aren't running git init, we should double check to make sure that we are in fact in a
    // git dir
    if let Some(Commands::Init {}) = &cli.command {
    } else {
        ensure!(is_current_git_directory(), "Not a git directory");
    }

    // SAFETY:
    // safe as long as clap sees that there is no argument passed into the function
    match &cli.command.unwrap() {
        Commands::Init {} => {
            info!("Initializing a git repo!");
            init_git_repo_in_current_dir()?;
        }
        Commands::CatFile { hash } => {
            info!("generating hash");
            cat_git_object(&hash)?;
        }
    }

    Ok(())
}
