use b::app;
use b::prelude::*;
use b::utils;

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
