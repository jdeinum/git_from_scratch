use crate::git::parsers::{
    parse_alpha, parse_content, parse_null, parse_space, parse_usize_string,
};
use anyhow::Result;
use bytes::Bytes;
use tracing::debug;

// Reads in the uncompressed bytes, and extracts the string contents
// The format of a blob object file looks: like this (after Zlib decompression):
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
pub fn parse_git_object_native(buf: Bytes) -> Result<(String, usize, Bytes)> {
    // parse the name
    let (current, otype) = parse_alpha(&buf, 0)?;
    debug!("Parsed object type: {otype}");

    // parse the first space
    let (current, _) = parse_space(&buf, current)?;
    debug!("Parsed space");

    // parse the content size
    let (current, content_size) = parse_usize_string(&buf, current)?;
    let content_size = content_size.parse::<usize>()?;
    debug!("Parsed content size: {content_size}");

    // parse the null byte
    let (current, _) = parse_null(&buf, current)?;

    // parse the content
    let (_, content) = parse_content(&buf, current, content_size)?;

    Ok((otype, content_size, content))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_git_object_native_native() {
        let input = b"blob 12\0Hello world!";
        let result = parse_git_object_native(Bytes::copy_from_slice(input));
        assert!(result.is_ok());
        let (obj_type, size, content) = result.unwrap();
        assert_eq!(obj_type, "blob".to_string());
        assert_eq!(size, 12);
        assert_eq!(content, Bytes::copy_from_slice(b"Hello world!"));
    }
}
