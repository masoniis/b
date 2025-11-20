pub mod active_gen_text;
pub mod camera_coords;
pub mod camera_xyz_coords;
pub mod fps_counter;
pub mod memory_counter;
pub mod mesh_counter;

pub use camera_coords::update_camera_chunk_coord_screen_text;
pub use camera_xyz_coords::update_camera_xyz_coord_screen_text;
pub use fps_counter::update_fps_counter_screen_text_system;
pub use memory_counter::{update_memory_counter_screen_text, SystemInfoResource};
pub use mesh_counter::{
    mesh_add_observer, mesh_remove_observer, update_mesh_counter_screen_text_system,
    MeshCounterResource,
};
