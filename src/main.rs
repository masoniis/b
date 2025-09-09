use tracing::{error, info, warn};

mod core;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logger::attach_logger();
    info!("Logger attached...");
    info!("Running app...");
    core::window::run_app()?;
    Ok(())
}
