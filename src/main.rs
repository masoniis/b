pub mod app;
pub mod ecs_core;
pub mod prelude;
pub mod render_world;
pub mod simulation_world;
pub mod utils;

pub use prelude::*;

#[instrument(skip_all, fields(name = "main"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    utils::logger::attach_logger();

    info!("Logger attached...");
    info!("Running app...");

    if let Err(e) = app::App::create_and_run() {
        error!("App error: {}", e);
    } else {
        info!("App runner finished without errors!");
    }

    Ok(())
}
