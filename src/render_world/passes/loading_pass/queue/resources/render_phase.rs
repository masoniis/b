use bevy_ecs::{entity::Entity, resource::Resource};

#[derive(Debug)]
pub struct PhaseItem {
    pub entity: Entity,
    pub distance: f32, // For sorting back-to-front
}

#[derive(Resource, Default)]
pub struct Opaque3dRenderPhase {
    pub items: Vec<PhaseItem>,
}
