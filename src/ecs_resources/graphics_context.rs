use crate::core::graphics::context::GraphicsContext;
use bevy_ecs::prelude::Resource;

// It's good practice to wrap it in a newtype struct.
#[derive(Resource)]
pub struct GraphicsContextResource {
    pub context: GraphicsContext,
}
