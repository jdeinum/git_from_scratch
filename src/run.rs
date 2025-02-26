use crate::{
    cli::{Cli, Commands},
    git::{
        LsTreeOptions, cat_git_object, hash_git_object, init_git_repo_in_current_dir, ls_tree_git,
        utils::is_current_git_directory,
    },
};
use anyhow::{Result, ensure};
use clap::Parser;
use std::path::Path;
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

    // get our stdout object
    let stdo = std::io::stdout().lock();

    // SAFETY:
    // safe as long as clap sees that there is no argument passed into the function
    match &cli.command.unwrap() {
        Commands::Init {} => {
            info!("Initializing a git repo!");
            init_git_repo_in_current_dir()?;
        }
        Commands::CatFile { hash } => {
            info!("generating hash");
            cat_git_object(hash, stdo)?;
        }
        Commands::HashFile {
            write_to_store,
            filename,
        } => {
            info!("generating git hash");
            hash_git_object(
                Path::new(&filename),
                write_to_store.unwrap_or(false).into(),
                stdo,
            )?;
        }
        Commands::LsTree { hash, name_only } => {
            info!("listing tree");

            ls_tree_git(
                hash,
                LsTreeOptions {
                    name_only: name_only.unwrap_or(false),
                },
                stdo,
            )?;
        }
    }

    Ok(())
}
