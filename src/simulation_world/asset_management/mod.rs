pub mod asset_storage;
pub mod mesh_asset;

pub use asset_storage::{Asset, AssetStorageResource, Handle};
pub use mesh_asset::MeshAsset;

// INFO: ---------------------------------
//         Asset Management Plugin
// ---------------------------------------

use crate::ecs_core::{EcsBuilder, Plugin};

pub struct AssetManagementPlugin;

impl Plugin for AssetManagementPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        builder.add_resource(AssetStorageResource::<MeshAsset>::default());
    }
}
