use tracing::{error, info, warn};
use tracing_subscriber;

mod core;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(true) // Show the module path
        .with_line_number(true) // Show the file and line number
        .with_thread_names(true) // Show the thread name
        .init(); // Set this as the global default subscriber

    info!("This is an informational message.");
    warn!("This is a warning!");
    error!("This is an error message.");

    core::window::run_app()?;
    Ok(())
}
