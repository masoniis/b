pub mod camera_control;
pub use camera_control::camera_control_system;

pub mod screen_diagnostics;
pub use screen_diagnostics::screen_diagnostics_system;

pub mod text_meshing;
pub use text_meshing::update_text_mesh_system;

pub mod time;
pub use time::time_system;

pub mod triangle_renderer;
pub use triangle_renderer::triangle_render_system;
