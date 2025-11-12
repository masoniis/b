use crate::prelude::*;
use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Default)]
pub struct TargetedBlock {
    pub position: Option<IVec3>,
}
