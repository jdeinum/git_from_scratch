use anyhow::{Result, ensure};
use bytes::Bytes;
use nom::AsChar;

pub fn parse_alpha(buf: &[u8], index: usize) -> Result<(usize, String)> {
    let mut x = buf
        .iter()
        .enumerate()
        .skip(index)
        .take_while(|(_, b)| b.as_char().is_alphabetic())
        .fold((index, String::new()), |mut acc, x| {
            acc.0 = x.0;
            acc.1.push(x.1.as_char());
            acc
        });
    x = (x.0 + 1, x.1); // move the index to the next character
    Ok(x)
}

pub fn parse_until_null(buf: &[u8], index: usize) -> Result<(usize, String)> {
    let mut x = buf
        .iter()
        .enumerate()
        .skip(index)
        .take_while(|(_, b)| b.as_char() != '\0')
        .fold((index, String::new()), |mut acc, x| {
            acc.0 = x.0;
            acc.1.push(x.1.as_char());
            acc
        });
    x = (x.0 + 1, x.1); // move the index to the next character
    Ok(x)
}

pub fn parse_usize_string(buf: &[u8], index: usize) -> Result<(usize, String)> {
    let mut x = buf
        .iter()
        .enumerate()
        .skip(index)
        .take_while(|(_, b)| b.as_char().is_numeric())
        .fold((index, String::new()), |mut acc, x| {
            acc.0 = x.0;
            acc.1.push(x.1.as_char());
            acc
        });
    x = (x.0 + 1, x.1); // move the index to the next character
    Ok(x)
}

pub fn parse_content(buf: &[u8], index: usize, size: usize) -> Result<(usize, Bytes)> {
    Ok((
        index + size,
        Bytes::copy_from_slice(&buf[index..index + size]),
    ))
}

pub fn parse_char(buf: &[u8], index: usize, c: char) -> Result<(usize, bool)> {
    ensure!(buf[index].as_char() == c);
    Ok((index + 1, true))
}

pub fn parse_space(buf: &[u8], index: usize) -> Result<(usize, bool)> {
    parse_char(buf, index, ' ')
}

pub fn parse_null(buf: &[u8], index: usize) -> Result<(usize, bool)> {
    parse_char(buf, index, '\0')
}
