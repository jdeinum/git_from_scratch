use anyhow::Result;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::io::Write;
use std::path::Path;
use tracing::info;

pub enum StoreHash {
    No,
    Yes,
}

impl From<bool> for StoreHash {
    fn from(value: bool) -> Self {
        match value {
            true => Self::Yes,
            false => Self::No,
        }
    }
}

// TODO: Pass in an ouput object to print to
pub fn run(filename: &Path, should_hash: StoreHash, mut w: impl Write) -> Result<()> {
    todo!()
}
