use anyhow::Context;
use anyhow::Result;
use std::sync::LazyLock;
use tempfile::TempDir;
use tracing::info;

pub static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TESTING_LOG").is_ok() {
        tracing_subscriber::fmt::init();
        info!("Initialized logging");
    }
});

pub fn create_and_swap_to_temp_dir() -> Result<TempDir> {
    let temp_dir = tempfile::TempDir::new().context("create new temp dir")?;
    std::env::set_current_dir(temp_dir.path()).context("change working dir to temp dir")?;
    Ok(temp_dir)
}
