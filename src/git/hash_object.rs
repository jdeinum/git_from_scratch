use crate::git::utils::GitObject;
use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;

pub enum StoreHash {
    No,
    Yes,
}

impl From<&bool> for StoreHash {
    fn from(value: &bool) -> Self {
        match value {
            true => Self::Yes,
            false => Self::No,
        }
    }
}

// TODO: Pass in an ouput object to print to
pub fn run(filename: &Path, should_hash: StoreHash, mut w: impl Write) -> Result<()> {
    let obj = GitObject::read_blob_from_file(filename).context("create blob from file")?;

    let hash = match should_hash {
        StoreHash::Yes => obj.write_to_objects().context("write blob to object store"),
        StoreHash::No => obj.write(std::io::sink()),
    }?;

    let str_hash = hex::encode(hash);
    w.write_all(str_hash.as_bytes())
        .context("write hash to writer")
}
