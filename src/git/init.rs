use anyhow::{Context, Result};
use std::io::Write;
use tracing::instrument;

#[instrument(err)]
pub fn run() -> Result<()> {
    // create the .git directory
    std::fs::create_dir(".git")
        .context("create .git directory")
        .context("create .git/")?;

    // create the .git/objects directory
    std::fs::create_dir(".git/objects")
        .context("create .git/objects directory")
        .context("create .git/objects/")?;

    // create the .git/refs directory
    std::fs::create_dir(".git/refs")
        .context("create .git/refs directory")
        .context("create .git/refs/")?;

    // create the HEAD file and write the contents to it
    let mut f = std::fs::File::open(".git/HEAD").context("open .git/HEAD")?;
    f.write_all(b"ref: refs/heads/main\n")
        .context("write refs to HEAD")?;

    Ok(())
}
