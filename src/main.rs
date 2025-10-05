pub mod ecs_core;

pub mod app;
pub mod game_world;
pub mod prelude;
pub mod render_world;
pub mod utils;

pub use prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
