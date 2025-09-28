pub mod core;
pub mod ecs_modules;
pub mod ecs_resources;
pub mod ecs_systems;
pub mod prelude;
pub mod utils;

pub use prelude::*;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     utils::logger::attach_logger();
//     info!("Logger attached...");
//     info!("Running app...");
//
//     if let Err(e) = core::app::App::run_app() {
//         error!("App error: {}", e);
//     } else {
//         info!("App runner finished without errors!");
//     }
//
//     Ok(())
// }

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We use pollster::block_on to run the async `run` function to completion.
    // This is the bridge between the synchronous native world and your async core logic.
    pollster::block_on(b::run());
    Ok(())
}

// You need an empty main for wasm builds when you have a [[bin]] section,
// otherwise the linker will complain.
#[cfg(target_arch = "wasm32")]
fn main() {
    // This does nothing on wasm, as `start()` in lib.rs is the entry point.
}
