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

pub fn create_object_and_return_hash(dir: &str, filename: &str) -> Result<String> {
    let hash = std::process::Command::new("git")
        .current_dir(dir)
        .arg("hash-object")
        .arg("-w")
        .arg(filename)
        .output()?
        .stdout;

    let hash = String::from_utf8(hash)?;
    debug!("file hash: {hash}");

    Ok(hash.trim().to_string())
}
