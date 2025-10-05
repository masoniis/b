use crate::render_world::context::GraphicsContext;
use bevy_ecs::prelude::Resource;

#[derive(Resource)]
pub struct GraphicsContextResource {
    pub context: GraphicsContext,
}
