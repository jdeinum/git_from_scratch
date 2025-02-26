use crate::git::{convert_file, git_objects_dir_exists, read_file};
use anyhow::{Result, ensure};
use std::io::Write;
use tracing::debug;

pub fn cat_git_object(hash: &str, mut w: impl Write) -> Result<()> {
    ensure!(git_objects_dir_exists(), ".git/objects/ does not exist");

    // read the file
    let buf = read_file(hash)?;

    debug!("buf: {buf:?}");

    // convert the file into UTF8
    let content = convert_file(buf)?;

    write!(w, "{}", content)?;

    Ok(())
}
