pub mod debug_screen;
pub mod update_fps_counter;
pub mod update_mesh_counter;

pub use debug_screen::{diagnostic_ui_is_visible, toggle_debug_diagnostics_system};
pub use update_fps_counter::update_fps_counter_system;
pub use update_mesh_counter::update_mesh_counter_system;
