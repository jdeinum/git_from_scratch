use crate::utils::TRACING;
use anyhow::Result;
use std::sync::LazyLock;

#[test]
pub fn test_git_init() -> Result<()> {
    // init tracing
    LazyLock::force(&TRACING);

    todo!();
}
