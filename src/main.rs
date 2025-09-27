use tracing::{error, info};

pub mod core;
pub mod ecs_modules;
pub mod ecs_resources;
pub mod ecs_systems;
pub mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logger::attach_logger();
    info!("Logger attached...");
    info!("Running app...");

    if let Err(e) = core::runner::run_app() {
        error!("App error: {}", e);
    } else {
        info!("App runner finished without errors!");
    }

    Ok(())
}
