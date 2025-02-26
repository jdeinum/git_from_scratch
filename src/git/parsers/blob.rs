use anyhow::Result;
use bytes::Bytes;
use nom::{
    AsChar, IResult, Parser,
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1, space1},
    combinator::map_res,
};
use tracing::debug;

// TODO: Return a reference to the passed buffer rather than create a copy of owned data
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

fn parse_object_type(buf: &Bytes, index: usize) -> Result<(usize, String)> {
    let mut x = buf
        .iter()
        .skip(index)
        .enumerate()
        .take_while(|(_, b)| b.as_char() != ' ')
        .fold((0, String::new()), |mut acc, x| {
            acc.0 = x.0 + index;
            acc.1.push(x.1.as_char());
            acc
        });

    // somereallylonggitobjecttypehere 12\0 hello world!
    //                               |
    // at this point, index is here  |
    // so we add 2 to it to move the index to the first character after the space
    x = (x.0 + 2, x.1);
    Ok(x)
}

fn parse_content_size(buf: &Bytes, index: usize) -> Result<(usize, usize)> {
    let x = buf
        .iter()
        .skip(index)
        .enumerate()
        .take_while(|(_, b)| b.as_char().is_numeric())
        .fold((index, String::new()), |mut acc, x| {
            acc.0 = index + x.0;
            acc.1.push(x.1.as_char());
            acc
        });

    // blob 12\0hello world!
    //       |
    // index is here
    // so we add 2 to it to move the index to the first character after the space
    let x = (x.0 + 2, x.1.parse::<usize>()?);
    Ok(x)
}

fn parse_content(buf: &Bytes, index: usize, size: usize) -> Result<(usize, Bytes)> {
    Ok((
        index + size,
        Bytes::copy_from_slice(&buf[index..index + size]),
    ))
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
pub fn parse_git_object_native(buf: Bytes) -> Result<(String, usize, Bytes)> {
    let (current, otype) = parse_object_type(&buf, 0)?;
    debug!("Parsed object type: {otype}");

    let (current, content_size) = parse_content_size(&buf, current)?;
    debug!("Parsed content size: {content_size}");

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
