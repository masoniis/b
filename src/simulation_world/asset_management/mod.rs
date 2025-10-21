pub mod asset_storage;
pub mod mesh_asset;

pub use asset_storage::{Asset, AssetStorageResource, Handle};
use bevy_ecs::{message::Messages, schedule::IntoScheduleConfigs};
pub use mesh_asset::{
    delete_stale_mesh_assets, update_mesh_ref_counts_system, MeshAsset,
    MeshComponentRemovedMessage, MeshDeletionRequest,
};

// INFO: ---------------------------------
//         Asset Management Plugin
// ---------------------------------------

use crate::{
    ecs_core::{EcsBuilder, Plugin},
    simulation_world::{asset_management::mesh_asset::MeshRefCounts, SimulationSchedule},
    SimulationSet,
};

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder
            .add_resource(AssetStorageResource::<MeshAsset>::default())
            .add_resource(MeshRefCounts::default())
            .init_resource::<Messages<MeshComponentRemovedMessage>>()
            .init_resource::<Messages<MeshDeletionRequest>>();

        builder
            .schedule_entry(SimulationSchedule::Main)
            .add_systems((
                update_mesh_ref_counts_system.in_set(SimulationSet::Update),
                delete_stale_mesh_assets.in_set(SimulationSet::RenderPrep),
            ));
    }
}
