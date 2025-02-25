use std::sync::LazyLock;
use tracing::info;

pub static TRACING: LazyLock<()> = LazyLock::new(|| {
    if std::env::var("TESTING_LOG").is_ok() {
        tracing_subscriber::fmt::init();
        info!("Initialized logging");
    }
});
