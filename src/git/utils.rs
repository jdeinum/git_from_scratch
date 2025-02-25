use super::parse_git_object_native;
use anyhow::{Context, Ok, Result, ensure};
use bytes::Bytes;
use flate2::bufread::ZlibDecoder;
use std::{
    io::{Read, Write},
    path::PathBuf,
};
use tracing::debug;

/// This module contains helper functions for reading and writing git objects

pub enum GitObjectType {
    Blob { buf: Bytes },
}

impl Write for GitObjectType {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
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

// Reads in the uncompressed bytes, and extracts the string contents
// The format of a blob object file looks like this (after Zlib decompression):
//
// blob <size>\0<content>
//
// <size> is the size of the content (in bytes)
//
// \0 is a null byte
//
// <content> is the actual content of the file
//
// For example, if the contents of a file are hello world, the blob object file would look like
// this (after Zlib decompression):
//
// blob 11\0hello world
pub fn convert_file(buf: Bytes) -> Result<String> {
    let (_, _, content) = parse_git_object_native(buf)?;
    Ok(String::from_utf8(content.to_vec())?)
}

pub fn write_file(_s: &str) -> Result<()> {
    todo!()
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
