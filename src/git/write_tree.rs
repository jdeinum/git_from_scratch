use anyhow::{Context, Result, bail};
use std::io::Write;
use walkdir::WalkDir;

pub fn run(mut w: impl Write) -> Result<()> {
    // iterate over the current directory
    for entry in WalkDir::new(".") {
        let entry = entry.context("extract path entry")?;

        match entry.path() {
            // if we are dealing with a file, we need to write the blob and return the sha
            x if x.is_file() => {}

            // if its a symlink, we'll just pass over it for know
            // TODO: Figure out what to do with symlinks
            x if x.is_symlink() => {}

            // if it's a directory, we'll want to recurse down and write all of those objects
            x if x.is_dir() => {}
            x => {
                bail!("don't know what the format of {x:?} is")
            }
        }
    }

    Ok(())
}
