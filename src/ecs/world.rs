use crate::ecs::resources::{DeltaTime, Input as InputResource};
use crate::graphics::camera::Camera;

pub struct World {
    pub input_resource: InputResource,
    pub delta_time: DeltaTime,
    pub camera: Camera,
    pub window_size: (u32, u32),
}

impl Default for World {
    fn default() -> Self {
        Self {
            input_resource: InputResource::new(),
            delta_time: DeltaTime::default(),
            camera: Camera::default(),
            window_size: (800, 600),
        }
    }
}
