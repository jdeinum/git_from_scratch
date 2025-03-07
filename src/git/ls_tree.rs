use crate::git::utils::{GitObject, GitObjectType};
use anyhow::{Context, Result, ensure};
use std::{
    ffi::CStr,
    io::{BufRead, Read, Write},
};
pub fn run(hash: &str, name_only: bool, mut w: impl Write) -> Result<()> {
    ensure!(name_only == true, "only know how to print names");

    // read in the object
    let mut obj = GitObject::read(hash).context("read git object in ls_tree")?;
    ensure!(
        obj.kind == GitObjectType::Tree,
        "ls_tree knows how to read only trees"
    );

    // read the entries from the buffer
    // git doesn't allow empty trees, so we don't have to worry about that case
    let mut buf: Vec<u8> = Vec::new();

    // we have a separate buffer thats exactly 20 bytes for the sha
    let mut sha_buf: [u8; 20] = [0; 20];

    loop {
        // clear the buffer
        buf.clear();

        // read until the null byte
        let n = obj
            .reader
            .read_until(0, &mut buf)
            .context("read mode and filename from buffer")?;

        // if we have read 0 bytes, then we're all done
        if n == 0 {
            break;
        }

        // convert the mode and name into a CStr
        let mode_and_name =
            CStr::from_bytes_with_nul(&buf[..n]).context("convert mode and name to CStr")?;

        // convert the CStr to a valid UTF-8 str
        let mode_and_name = mode_and_name
            .to_str()
            .context("convert CStr to UTF-8 str")?;

        // parse the mode and name
        let (_mode, name) = mode_and_name
            .split_once(' ')
            .context("no space in tree entry")?;

        // read the sha
        // TODO: I would like to use take, but it consumes the reader that its reading from
        // running into ownership issues
        obj.reader
            .read_exact(&mut sha_buf)
            .context("read 20 byte tree sha entry")?;

        // write to the output stream
        if name_only {
            writeln!(w, "{name}").context("write name to writer")?;
        }
    }

    Ok(())
}
