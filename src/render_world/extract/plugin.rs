use super::{
    clone_resource_system, extract_meshes::MeshEntityMap, extract_meshes_system,
    extract_resource_system, RenderCameraResource, RenderMeshStorageResource, RenderTimeResource,
};
use crate::{
    game_world::global_resources::{AssetStorageResource, MeshAsset},
    prelude::*,
    render_world::RenderSchedule,
};
use bevy_ecs::prelude::*;

pub struct ExtractModulePlugin;

impl Plugin for ExtractModulePlugin {
    fn build(&self, schedules: &mut ScheduleBuilder, world: &mut World) {
        world.insert_resource(RenderTimeResource::default());
        world.insert_resource(RenderCameraResource::default());
        world.insert_resource(MeshEntityMap::default());
        world.insert_resource(RenderMeshStorageResource::default());

        schedules.entry(RenderSchedule::Extract).add_systems((
            extract_resource_system::<RenderTimeResource>,
            extract_resource_system::<RenderCameraResource>,
            clone_resource_system::<AssetStorageResource<MeshAsset>>,
            extract_meshes_system,
        ));
    }
}
