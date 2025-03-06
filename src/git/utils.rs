use crate::git::parsers::*;
/** This module contains helper functions for reading and writing git objects
*
**/
use anyhow::{Context, Ok, Result, ensure};
use bytes::Bytes;
use flate2::bufread::ZlibDecoder;
use std::{
    fmt,
    io::{BufRead, Read},
    path::{Path, PathBuf},
};
use tracing::debug;

pub(crate) enum GitObjectType {
    Blob,
    Tree,
    Commit,
}

impl fmt::Display for GitObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitObjectType::Blob => write!(f, "blob"),
            GitObjectType::Tree => write!(f, "tree"),
            GitObjectType::Commit => write!(f, "commit"),
        }
    }
}

// the main idea behind this is that different methods will return a git object with the correct
// kind, and a reader that you can use to read the remaining bytes fr with the correct kind, and a
// reader that you can use to read the remaining bytes from.
pub(crate) struct GitObject<R> {
    kind: GitObjectType,
    expected_length: usize,
    reader: R,
}

impl GitObject<()> {
    pub(crate) fn read_blob_from_file(
        filename: impl AsRef<Path>,
    ) -> Result<GitObject<impl BufRead>> {
        todo!()
    }

    pub(crate) fn read(hash: &str) -> Result<GitObject<impl BufRead>> {
        // first we need to split the hash into its dir and remaining bytes

        Ok(GitObject {
            kind: GitObjectType::Blob,
            expected_length: 10,
            reader: Vec::new(),
        })
    }
}

// Returns the uncompressed bytes from the file associated with the hash
pub fn read_file(hash: &str) -> Result<Bytes> {
    // ensure the .git/objects directory exists
    ensure!(git_objects_dir_exists(), "No git objects directory");

    // split up the hash into the directory and the filename
    let (directory, filename) = hash.split_at(2);

    // read the file
    let file: Bytes = {
        let p = format!(".git/objects/{}/{}", directory, filename);
        let f = std::fs::read(&p).with_context(|| format!("File does not exist: {p}"))?;
        Bytes::from(f)
    };

    debug!("read {} bytes from {hash}", file.len());

    // decompress using gzip
    let mut decompressed_bytes = Vec::new();
    let mut decoder = ZlibDecoder::new(file.as_ref());
    decoder.read_to_end(&mut decompressed_bytes)?;

    debug!("got {} decompressed bytes", decompressed_bytes.len());

    // return the uncompressed bytes
    Ok(decompressed_bytes.into())
}

pub fn convert_file(buf: Bytes) -> Result<String> {
    let (_, _, content) = parse_git_object_native(buf)?;
    Ok(String::from_utf8(content.to_vec())?)
}

pub fn is_current_git_directory() -> bool {
    std::path::Path::new(".git/").exists()
}

pub fn git_objects_dir_exists() -> bool {
    std::path::Path::new(".git/objects").exists()
}

pub fn create_directory(root_dir: PathBuf, name: &str) -> Result<()> {
    let full_path = {
        let mut x = root_dir.clone();
        x.push(name);
        x
    };

    std::fs::create_dir(full_path).map_err(|e| e.into())
}
