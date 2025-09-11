use crate::ecs::systems::System;
use crate::ecs::world::World;

pub struct CameraUpdateSystem;
impl System for CameraUpdateSystem {
    fn new_events_hook(&mut self, world: &mut World) {
        let aspect_ratio = world.window_size.0 as f32 / world.window_size.1 as f32;
        world.camera.update_view_matrix();
        world.camera.update_projection_matrix(aspect_ratio);
    }
}
