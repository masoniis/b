use tracing::{error, info};

pub mod core;
pub mod graphics;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logger::attach_logger();
    info!("Logger attached...");
    info!("Running app...");

    if let Err(e) = core::window::run_app() {
        error!("App error: {}", e);
    }

    Ok(())
}
