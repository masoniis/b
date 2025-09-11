use crate::ecs::resources::{DeltaTime, Input as InputResource};
use crate::ecs::resources::Camera;
use crate::graphics::renderer::Renderer;
use crate::graphics::shaders::shader_program::ShaderProgram;

pub struct World {
    pub input_resource: InputResource,
    pub delta_time: DeltaTime,
    pub camera: Camera,
    pub window_size: (u32, u32),
    pub renderer: Option<Renderer>,
    pub shader_program: Option<ShaderProgram>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            input_resource: InputResource::new(),
            delta_time: DeltaTime::default(),
            camera: Camera::default(),
            window_size: (800, 600),
            renderer: None,
            shader_program: None,
        }
    }
}
