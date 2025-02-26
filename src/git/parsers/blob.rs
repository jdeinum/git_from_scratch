use crate::git::parsers::{parse_alpha, parse_content, parse_usize_string};
use anyhow::Result;
use bytes::Bytes;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1, space1},
    combinator::map_res,
};
use tracing::debug;

// TODO: Return a reference to the passed buffer rather than create a copy of owned data
#[allow(unused)]
pub fn parse_git_object<'a>(buf: &'a [u8]) -> IResult<&'a [u8], (String, usize, Bytes)> {
    // Parse the object type
    let (b, obj_type) = map_res(alpha1, |x: &[u8]| String::from_utf8(x.to_vec())).parse(buf)?;

    // Parse the space after the object type
    let (b, _) = space1.parse(b)?;

    // Parse the size as a string of digits and convert it to a usize
    let (b, size) = map_res(digit1, |x: &[u8]| {
        String::from_utf8(x.to_vec()).unwrap().parse::<usize>()
    })
    .parse(b)?;

    // Parse the null byte after the size
    let (b, _) = tag("\0").parse(b)?;

    // Parse the content, which is exactly `size` bytes long
    let (b, content) = take(size)(b)?;

    // Return the result as a tuple (object type, size, content)
    // Convert the content into Bytes for ownership
    Ok((b, (obj_type, size, Bytes::copy_from_slice(content))))
}

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
    let (current, otype) = parse_alpha(&buf, 0)?;
    debug!("Parsed object type: {otype}");

    // move the current pointer 1 past the space
    let current = current + 1;

    let (current, content_size) = parse_usize_string(&buf, current)?;
    let content_size = content_size.parse::<usize>()?;
    debug!("Parsed content size: {content_size}");

    // move the current pointer 1 past the null byte
    let current = current + 1;

    let (_, content) = parse_content(&buf, current, content_size)?;

    Ok((otype, content_size, content))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_git_object() {
        let input = b"blob 12\0Hello world!";
        let result = parse_git_object(input);
        assert!(result.is_ok());
        let (_, (obj_type, size, content)) = result.unwrap();
        assert_eq!(obj_type, "blob".to_string());
        assert_eq!(size, 12);
        assert_eq!(content, Bytes::copy_from_slice(b"Hello world!"));
    }

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
