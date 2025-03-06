use super::parsers::parse_git_tree;
use anyhow::Result;
use itertools::*;
use std::io::Write;
use tracing::debug;

pub fn run(hash: &str, name_only: bool, mut w: impl Write) -> Result<()> {
    todo!()
}
