use crate::utils::TRACING;
use anyhow::Result;
use std::sync::LazyLock;

#[test]
pub fn test_git_write_tree() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    Ok(())
}
