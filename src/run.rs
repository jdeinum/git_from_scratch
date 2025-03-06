use crate::{
    cli::{Cli, Commands},
    git,
};
use anyhow::{Context, Result};
use clap::Parser;
use std::path::Path;

pub fn run() -> Result<()> {
    // parse will error if no sub command is passed in
    let cli = Cli::parse();

    // get our stdout object
    let stdo = std::io::stdout().lock();

    // SAFETY:
    // safe as long as clap sees that there is no argument passed into the function
    match &cli.command.context("match command")? {
        Commands::Init {} => git::init::run(),
        Commands::CatFile { hash, pretty_print } => {
            git::cat_file::run(hash.trim(), pretty_print.into(), stdo)
        }
        Commands::HashObject {
            write_to_store,
            filename,
        } => git::hash_object::run(Path::new(&filename), write_to_store.into(), stdo),
        Commands::LsTree { hash, name_only } => {
            git::ls_tree::run(hash.trim(), name_only.unwrap_or(false), stdo)
        }
        Commands::WriteTree {} => git::write_tree::run(stdo),
    }
    .context("run command")
}
