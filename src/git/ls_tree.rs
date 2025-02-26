use anyhow::Result;
use std::io::Write;

pub struct LsTreeOptions {
    pub name_only: bool,
}

pub fn ls_tree_git(hash: &str, options: LsTreeOptions, mut w: impl Write) -> Result<()> {
    todo!()
}
