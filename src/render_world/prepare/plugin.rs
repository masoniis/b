use crate::{
    core::world::{EcsBuilder, Plugin},
    render_world::extract::resources::RenderMeshStorageResource,
    render_world::RenderSchedule,
};

use super::{
    resources::mesh_pipeline::MeshPipelineLayoutsResource, systems::prepare_meshes_system,
};

pub struct PrepareModulePlugin;

impl Plugin for PrepareModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(RenderMeshStorageResource::default());
        builder.from_world_resource::<MeshPipelineLayoutsResource>();

        builder
            .schedule_entry(RenderSchedule::Prepare)
            .add_systems(prepare_meshes_system);
    }
}
