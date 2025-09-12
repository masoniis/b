// INFO: -----------------
//         EXTERNAL
// -----------------------
mod external;
pub use external::InputSystem;

// INFO: -------------
//         MAIN
// -------------------
mod main;
pub use main::{camera_control_system, time_system};

// INFO: ---------------
//         RENDER
// ---------------------
mod render;
pub use render::render_system;

// INFO: ----------------
//         STARTUP
// ----------------------
mod startup;
pub use startup::setup_chunk_system;
