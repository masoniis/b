use super::{systems::prepare_meshes_system, MeshPipelineLayoutsResource};
use crate::{
    core::world::{EcsBuilder, Plugin},
    render_world::extract::resources::RenderMeshStorageResource,
    render_world::RenderSchedule,
};

pub struct PrepareModulePlugin;

impl Plugin for PrepareModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(RenderMeshStorageResource::default());
        builder.init_resource::<MeshPipelineLayoutsResource>();

        builder
            .schedule_entry(RenderSchedule::Prepare)
            .add_systems(prepare_meshes_system);
    }
}
