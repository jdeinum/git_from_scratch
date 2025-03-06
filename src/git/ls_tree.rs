use crate::git::utils::{GitObject, GitObjectType};
use anyhow::{Context, Result, ensure};
use std::io::{BufRead, Write};
pub fn run(hash: &str, name_only: bool, mut w: impl Write) -> Result<()> {
    // read in the object
    let obj = GitObject::read(hash).context("read git object in ls_tree")?;
    ensure!(
        obj.kind == GitObjectType::Tree,
        "ls_tree knows how to read only trees"
    );

    // read the entries from the buffer
    // git doesn't allow empty trees, so we don't have to worry about that case
    let mut buf: Vec<u8> = Vec::new();
    loop {
        // parse until null byte, which includes the mode, and the file name
        let mode_and_filename = obj
            .reader
            .read_until(0, &mut buf)
            .context("read mode and filename from buffer")?;
    }

    Ok(())
}
