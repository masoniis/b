pub mod core;
pub mod ecs_modules;
pub mod ecs_resources;
pub mod ecs_systems;
pub mod prelude;
pub mod utils;

// Import your project modules
use tracing::{error, info};

// This is the shared entry point for both native and web.
pub async fn run() {
    // Platform-specific logger setup
    #[cfg(not(target_arch = "wasm32"))]
    {
        utils::logger::native_logger::attach_logger();
    }
    #[cfg(target_arch = "wasm32")]
    {
        // For wasm, we need to use a different logger
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(tracing::log::Level::Info)
            .expect("Couldn't initialize logger");
    }

    info!("Logger attached...");
    info!("Running app...");

    // This is where you would create your window and run the app.
    // This logic itself needs to be async, which you've likely done in your App.
    // Let's assume you have a function that sets up and runs the app async.
    core::app::run().await;
}

// This is the web entry point.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn start() {
    // The `wasm_bindgen(start)` attribute tells wasm-bindgen to call this function
    // automatically when the Wasm module is loaded.
    run().await;
}
