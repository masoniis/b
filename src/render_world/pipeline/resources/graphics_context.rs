use crate::core::graphics::context::GraphicsContext;
use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct GraphicsContextResource {
    pub context: GraphicsContext,
}
