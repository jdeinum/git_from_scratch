use anyhow::Result;
use bytes::Bytes;
use nom::AsChar;

pub fn parse_alpha(buf: &[u8], index: usize) -> Result<(usize, String)> {
    let mut x = buf
        .iter()
        .skip(index)
        .enumerate()
        .take_while(|(_, b)| b.as_char().is_alphabetic())
        .fold((0, String::new()), |mut acc, x| {
            acc.0 = x.0 + index;
            acc.1.push(x.1.as_char());
            acc
        });

    // somereallylonggitobjecttypehere 12\0 hello world!
    //                               |
    // at this point, index is here  |
    // so we add 1 to it to move the index to the first character after word
    x = (x.0 + 1, x.1);
    Ok(x)
}

pub fn parse_usize_string(buf: &[u8], index: usize) -> Result<(usize, String)> {
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
    // so we add 1 to it to move the index to the first byte after the number
    let x = (x.0 + 1, x.1);
    Ok(x)
}

pub fn parse_content(buf: &[u8], index: usize, size: usize) -> Result<(usize, Bytes)> {
    Ok((
        index + size,
        Bytes::copy_from_slice(&buf[index..index + size]),
    ))
}
