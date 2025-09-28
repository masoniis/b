pub mod core;
pub mod ecs_bridge;
pub mod ecs_modules;
pub mod ecs_resources;
pub mod prelude;
pub mod utils;

pub use prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logger::attach_logger();
    info!("Logger attached...");
    info!("Running app...");

    if let Err(e) = core::app::App::run_app() {
        error!("App error: {}", e);
    } else {
        info!("App runner finished without errors!");
    }

    Ok(())
}
