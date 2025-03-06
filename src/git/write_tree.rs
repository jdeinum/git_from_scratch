use anyhow::{Result, ensure};
use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// A trees SHA1 comes from hashing its tree object file
///
/// ------------------------------------------------------
/// The format of a tree object file looks like this (after Zlib decompression):
///
///  tree <size>\0
///  <mode> <name>\0<20_byte_sha>
///  <mode> <name>\0<20_byte_sha>
///
/// Where mode is one of the following numbers:
/// 100644 (regular file)
/// 100755 (executable file)
/// 120000 (symbolic link)
///
/// And name is the file or directory name
///
/// NOTE: The SHA is not necessarily valid UTF-8
/// ------------------------------------------------------
///
/// Assume the following directory structure for the explanation of writing:
///
///             foo
///            /   \
///          bar     foo.txt
///        /    \
/// foo_bar.txt  baz
///               \
///                foo_bar_baz.txt
///
///
///
/// The approximate algorithm for generating the SHA1 for foo/ is as follows:
///
/// 1. Generate blob file and SHA1 for foo_bar_baz.txt
/// 2. Generate tree file and SHA1 for baz/
/// 3. Generate blob file and SHA1 for foo_bar.txt
/// 4. Generate tree file and SHA1 for bar/
/// 5. Generate blob file and SHA1 for foo.txt
/// 6. Generate tree file and SHA1 for foo/
///
///
/// Essentially we have a topological sort where there is an edge between Tree A to Tree B if A
/// contains B (i.e B is a subtree of A)
///
/// So our general strategy will be as follows:
///
/// 1. Form a DAG of nodes
/// 2. Take any node with no outgoing edges
/// 3. For all files in that node, generate blob files and SHA1 if they don't exist
/// 4. Generate a tree file and SHA1 for the node
/// 5. Repeat [2-4] until all nodes are explored

/// Generates our dag of directories to iterate over and generate tree files and SHAs for
///
/// IMPORTANT: We assume there are no cycles
///
/// acc will contain
fn generate_file_dag(p: &Path, acc: &mut Vec<String>) -> Result<()> {
    if p.is_dir() {
        for entry in fs::read_dir(p)? {
            let entry = entry?;
            let mut path = PathBuf::from(p);
            path.push(entry.path());
            if path.is_dir() {
                let full_path = path.to_string_lossy().to_string();
                acc.push(full_path);
                generate_file_dag(&path, acc)?;
            }
        }
    }
    Ok(())
}

fn traverse_file_dag(paths: Vec<String>, f: &dyn Fn(String) -> Result<()>) -> Result<()> {
    for p in paths {
        f(p)?;
    }

    Ok(())
}

fn create_git_object(p: String, m: &mut HashMap<String, String>) -> Result<()> {
    let full_path = Path::new(&p);

    // make sure our path actually exists
    ensure!(full_path.exists(), "path does not exist");

    // if the path is a directory we'll create a tree
    // otherwise, we'll create a blob
    if full_path.is_dir() {
        create_git_tree(p, m)?;
    } else {
        create_git_blob(p, m)?;
    }

    Ok(())
}

fn create_git_blob(p: String, m: &mut HashMap<String, String>) -> Result<String> {
    // if the path already exists in the map, return the contained SHA1
    if let Some(s) = m.get(&p) {
        return Ok(s.to_string());
    }

    // if its not in the map, either it was written previously or we've never seen it.
    // in either case, we'll have to create the blob itself
    todo!()
}

fn create_git_tree(p: String, m: &mut HashMap<String, String>) -> Result<String> {
    todo!()
}

pub fn run(mut w: impl Write) -> Result<()> {
    todo!()
}
