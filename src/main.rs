use git_from_scratch::run;
use tracing::error;

fn main() {
    // init tracing
    tracing_subscriber::fmt::init();

    // run the app
    if let Err(e) = run::run() {
        error!("Error: {e}");
    }
}
