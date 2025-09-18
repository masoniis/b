use tracing::{error, info};

pub mod core;
pub mod ecs;
pub mod graphics;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logger::attach_logger();
    info!("Logger attached...");
    info!("Running app...");

    if let Err(e) = core::window::run_app().await {
        error!("App error: {}", e);
    }

    Ok(())
}
