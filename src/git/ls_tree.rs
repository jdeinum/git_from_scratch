use super::parsers::parse_git_tree;
use crate::git::utils::read_file;
use anyhow::Result;
use itertools::*;
use std::io::Write;

pub struct LsTreeOptions {
    pub name_only: bool,
}

pub fn ls_tree_git(hash: &str, options: LsTreeOptions, mut w: impl Write) -> Result<()> {
    // read in the bytes
    let decompressed_bytes = read_file(hash)?;

    // parse the tree
    let tree = parse_git_tree(decompressed_bytes)?;

    // now that we have the tree, print it however we want
    // for now, we'll just use debug print for standard printing but make the name-only as expected
    let mut s: String = String::new();

    // change how we print things
    match options.name_only {
        false => s.push_str(&format!("{tree:?}")),
        true => {
            let names = tree
                .entries
                .iter()
                .map(|x| x.name.clone())
                .sorted()
                .join("\n");
            s.push_str(&format!("{names}\n"));
        }
    }

    // print things
    writeln!(&mut w, "{}", s)?;

    Ok(())
}
