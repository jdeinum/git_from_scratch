use crate::git::{GitTree, TreeEntry};
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
    let mut current = 0;

    let (header_finish, content_size) = parse_tree_header(buf)?;
    current = header_finish;

    while current < content_size.0 + header_finish {
        let (c, entry) = parse_single_tree_entry(buf, current)?;
        res.push(entry);
        current = c;
    }

    Ok(GitTree { entries: res })
}

struct TreeContentSize(usize);
fn parse_tree_header(buf: &[u8]) -> Result<(usize, TreeContentSize)> {
    // parse the tree keyword

    // parse the size

    // parse the null byte

    // move the current cursor one position past the null byte

    todo!()
}

fn parse_single_tree_entry(buf: &[u8], start: usize) -> Result<(usize, TreeEntry)> {
    // parse the mode

    // parse the space

    // parsr the name

    // parse the sha

    todo!()
}
