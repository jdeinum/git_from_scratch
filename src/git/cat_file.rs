use crate::git::utils::GitObject;
use anyhow::{Context, Result, ensure};
use std::io::Write;

#[derive(PartialEq, PartialOrd, Debug)]
pub enum PrettyPrint {
    Yes,
    No,
}

impl From<&bool> for PrettyPrint {
    fn from(value: &bool) -> Self {
        match value {
            false => Self::No,
            true => Self::Yes,
        }
    }
}

pub fn run(hash: &str, pretty_print: PrettyPrint, mut w: impl Write) -> Result<()> {
    ensure!(
        pretty_print == PrettyPrint::Yes,
        "not sure how to not pretty print"
    );

    // read the object
    let mut obj = GitObject::read(hash).context("get git object from hash")?;

    std::io::copy(&mut obj.reader, &mut w).context("copying contents of object to writer")?;

    Ok(())
}
