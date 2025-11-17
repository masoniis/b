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
            mesh_ref_count_add_observer, mesh_ref_count_remove_observer, MeshRefCounts,
            OpaqueMeshShadow,
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
        builder.add_resource(AssetStorageResource::<MeshAsset>::default());
        builder.add_resource(OpaqueMeshShadow::default());

        // mesh ref count tracking
        builder
            .add_resource(MeshRefCounts::default())
            .add_observer(mesh_ref_count_add_observer)
            .add_observer(mesh_ref_count_remove_observer);

        // mesh deletion handling
        builder
            .init_resource::<Messages<MeshDeletionRequest>>()
            .schedule_entry(SimulationSchedule::Main)
            .add_systems(delete_stale_mesh_assets.in_set(SimulationSet::RenderPrep));
    }
}
