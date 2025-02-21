use anyhow::Result;
use std::sync::LazyLock;
use temp_testdir::TempDir;
use tracing::instrument;

// create a new temporary directory and
pub fn create_tmpdir_and_chenv() -> Result<String> {
    // create our temporary directory
    let test_dir = TempDir::default();

    // change working dir to test dir
    std::env::set_current_dir(&test_dir)?;

    // return the path to h
    Ok(test_dir.to_string_lossy().to_string())
}

pub static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TESTING_LOG").is_ok() {
        tracing_subscriber::fmt::init();
    }
});
