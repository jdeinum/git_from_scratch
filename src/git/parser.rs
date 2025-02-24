use bytes::Bytes;
use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
    character::complete::{alpha1, digit1, space1},
    combinator::map_res,
};

// TODO: Return a reference to the passed buffer rather than create a copy of owned data
pub fn parse_git_object<'a>(buf: &'a [u8]) -> IResult<&'a [u8], (&'a [u8], usize, &'a [u8])> {
    // Parse the object type (e.g., "blob", "tree", "commit", "tag")
    let (buf, obj_type) = alpha1.parse(buf)?;

    // Parse the space after the object type
    let (buf, _) = space1(buf)?;

    // Parse the size as a string of digits and convert it to a usize
    let (buf, size) = map_res(digit1, |x: &[u8]| {
        std::str::from_utf8(x).unwrap().parse::<usize>()
    })
    .parse(buf)?;

    // Parse the null byte after the size
    let (buf, _) = tag("\0")(buf)?;

    // Parse the content, which is exactly `size` bytes long
    let (buf, content) = take(size)(buf)?;

    Ok((buf, (obj_type, size, content)))
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
        assert_eq!(obj_type, b"blob");
        assert_eq!(size, 12);
        assert_eq!(content, Bytes::copy_from_slice(b"Hello world!"));
    }
}
