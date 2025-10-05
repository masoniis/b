use super::{
    clone_resource_system, extract_meshes::MeshEntityMap, extract_meshes_system,
    extract_resource_system, RenderCameraResource, RenderMeshStorageResource, RenderTimeResource,
};
use crate::{
    core::world::{EcsBuilder, Plugin},
    game_world::global_resources::{AssetStorageResource, MeshAsset},
    render_world::RenderSchedule,
};

pub struct ExtractModulePlugin;

impl Plugin for ExtractModulePlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(RenderTimeResource::default());
        builder.add_resource(RenderCameraResource::default());
        builder.add_resource(MeshEntityMap::default());
        builder.add_resource(RenderMeshStorageResource::default());

        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                extract_resource_system::<RenderTimeResource>,
                extract_resource_system::<RenderCameraResource>,
                clone_resource_system::<AssetStorageResource<MeshAsset>>,
                extract_meshes_system,
            ));
    }
}
