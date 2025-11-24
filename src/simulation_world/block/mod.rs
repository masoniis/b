pub mod block_definition;
pub mod block_registry;
pub mod targeted_block;

pub use block_definition::{
    load_block_from_str, BlockDescription, BlockFaceTextures, BlockRenderData,
};
pub use block_registry::{BlockId, BlockRegistryResource, AIR_BLOCK_ID, SOLID_BLOCK_ID};
pub use targeted_block::TargetedBlock;

// INFO: ----------------------
//         Block plugin
// ----------------------------

use crate::prelude::*;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // insert resources
        builder
            .init_resource::<BlockRegistryResource>()
            .init_resource::<TargetedBlock>();
    }
}
