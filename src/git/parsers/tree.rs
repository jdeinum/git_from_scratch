use crate::git::parsers::{parse_alpha, parse_content, parse_usize_string};
use crate::git::utils::{GitTree, TreeEntry};
use anyhow::{Result, ensure};
use bytes::Bytes;

pub fn parse_git_tree(buf: Bytes) -> Result<GitTree> {
    parse_tree_internal(&buf)
}

// The format of a tree object file looks like this (after Zlib decompression):
//
//   tree <size>\0
//   <mode> <name>\0<20_byte_sha>
//   <mode> <name>\0<20_byte_sha>
//
// Where mode is one of the following numbers:
// 100644 (regular file)
// 100755 (executable file)
// 120000 (symbolic link)
//
// And name is the file or directory name
//
// NOTE: The SHA is not necessarily valid UTF-8
fn parse_tree_internal(buf: &[u8]) -> Result<GitTree> {
    let mut res: Vec<TreeEntry> = Vec::new();

    let (header_finish, content_size) = parse_tree_header(buf)?;
    let mut current = header_finish;

    while current < content_size.0 + header_finish {
        let (c, entry) = parse_single_tree_entry(buf, current)?;
        res.push(entry);
        current = c;
    }

    Ok(GitTree { entries: res })
}

struct TreeContentSize(usize);
fn parse_tree_header(buf: &[u8]) -> Result<(usize, TreeContentSize)> {
    let current = 0;

    // parse the tree keyword
    let (current, otype) = parse_alpha(buf, current)?;
    ensure!(otype == "tree".to_string(), "expecting tree as object type");

    // move the cursor 1 past the space
    let current = current + 1;

    // parse the size
    let (current, content_size) = parse_usize_string(buf, current)?;

    // move the cursor 1 past the null byte
    let current = current + 1;

    Ok((current, TreeContentSize(content_size.parse()?)))
}

fn parse_single_tree_entry(buf: &[u8], start: usize) -> Result<(usize, TreeEntry)> {
    // parse the mode
    let (current, mode) = parse_usize_string(buf, start)?;

    // move the cursor 1 past the space
    let current = current + 1;

    // parse the name
    let (current, name) = parse_alpha(buf, current)?;

    // move the cursor 1 past the null byte
    let current = current + 1;

    // parse the sha
    let (current, sha) = parse_content(buf, current, 20)?;

    Ok((current, TreeEntry { name, mode, sha }))
}
