use anyhow::Result;
use std::sync::LazyLock;
use tracing::debug;
use tracing::info;

pub static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TESTING_LOG").is_ok() {
        tracing_subscriber::fmt::init();
        info!("Initialized logging");
    }
});

pub fn create_object_and_return_hash(path: &str) -> Result<String> {
    let hash = std::process::Command::new("git")
        .current_dir(path)
        .arg("hash-object")
        .arg("-w")
        .arg("test.txt")
        .output()?
        .stdout;

    let hash = String::from_utf8(hash)?;
    debug!("file hash: {hash}");

    Ok(hash.trim().to_string())
}
