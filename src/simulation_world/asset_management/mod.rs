pub mod asset_storage;
pub mod mesh_asset;
pub mod texture_map_registry;

pub use asset_storage::{Asset, AssetStorageResource, Handle};
pub use mesh_asset::{delete_stale_mesh_assets, MeshAsset, MeshDeletionRequest};
pub use texture_map_registry::TextureMapResource;

// INFO: ---------------------------------
//         Asset Management Plugin
// ---------------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{
        asset_management::mesh_asset::{
            opaque_mesh_added_observer, opaque_mesh_removed_observer,
            transparent_mesh_added_observer, transparent_mesh_removed_observer, MeshRefCounts,
            OpaqueMeshShadow, TransparentMeshShadow,
        },
        SimulationSchedule,
    },
    SimulationSet,
};
use bevy_ecs::{message::Messages, schedule::IntoScheduleConfigs};

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // the mesh asset storage
        builder
            .add_resource(AssetStorageResource::<MeshAsset>::default())
            .add_resource(TransparentMeshShadow::default())
            .add_resource(OpaqueMeshShadow::default());

        // mesh ref count tracking
        builder
            .add_resource(MeshRefCounts::default())
            .add_observer(opaque_mesh_added_observer)
            .add_observer(opaque_mesh_removed_observer)
            .add_observer(transparent_mesh_added_observer)
            .add_observer(transparent_mesh_removed_observer);

        // mesh deletion handling
        builder
            .init_resource::<Messages<MeshDeletionRequest>>()
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(delete_stale_mesh_assets.in_set(SimulationSet::RenderPrep));
    }
}
