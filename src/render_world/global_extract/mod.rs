pub mod generic_systems;
pub mod resources;
pub mod utils;

pub use generic_systems::*;
pub use resources::*;
pub use utils::*;

// INFO: ---------------------------
//         Plugin definition
// ---------------------------------

use crate::{
    ecs_core::{
        state_machine::{AppState, GameState},
        EcsBuilder, Plugin,
    },
    render_world::scheduling::RenderSchedule,
    simulation_world::{
        asset_management::{AssetStorageResource, MeshAsset},
        input::resources::WindowSizeResource,
    },
};
use bevy_ecs::prelude::*;

pub struct SimulationExtractionPlugin;

impl Plugin for SimulationExtractionPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // Extraction here is for global resources used across
        // many different render systems.
        //
        // Anything specific to a pass or otherwise should be
        // located in that pass's dedicated plugin.
        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                (
                    clone_resource_system::<AssetStorageResource<MeshAsset>>,
                    extract_resource_system::<RenderTimeResource>,
                    (extract_resource_system::<RenderWindowSizeResource>)
                        .run_if(simulation_world_resource_changed::<WindowSizeResource>),
                    extract_state_system::<GameState>,
                    extract_state_system::<AppState>,
                ),
                extract_active_camera_system,
            ));
    }
}
