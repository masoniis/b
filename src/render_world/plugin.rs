use super::{
    extract::{
        extract_meshes::MeshEntityMap, RenderCameraResource, RenderMeshStorageResource,
        RenderTimeResource,
    },
    render::render_scene_system,
    RenderSchedule,
};
use crate::{
    game_world::{
        app_lifecycle::{AppState, GameState},
        global_resources::{AssetStorageResource, MeshAsset},
        state_machine::StatePlugin,
    },
    prelude::*,
    render_world::{
        extract::{self},
        prepare::{self, MeshPipelineLayoutsResource},
        queue::{self, RenderQueueResource},
    },
};

/// The main render world plugin that orchestrates all the phases of rendering
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // Resources
        builder.init_resource::<RenderTimeResource>();
        builder.init_resource::<RenderCameraResource>();
        builder.init_resource::<MeshEntityMap>();
        builder.init_resource::<RenderMeshStorageResource>();
        builder.init_resource::<RenderQueueResource>();
        builder.init_resource::<MeshPipelineLayoutsResource>();

        // Plugin dependencies
        builder.add_plugin(StatePlugin::<AppState>::default());
        builder.add_plugin(StatePlugin::<GameState>::default());

        // System builders
        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                extract::extract_resource_system::<RenderTimeResource>,
                extract::extract_resource_system::<RenderCameraResource>,
                extract::clone_resource_system::<AssetStorageResource<MeshAsset>>,
                extract::extract_state_system::<AppState>,
                extract::extract_state_system::<GameState>,
                extract::extract_meshes_system,
            ));

        builder
            .schedule_entry(RenderSchedule::Prepare)
            .add_systems(prepare::prepare_meshes_system);

        builder
            .schedule_entry(RenderSchedule::Queue)
            .add_systems(queue::queue_mesh_system);

        builder
            .schedule_entry(RenderSchedule::Render)
            .add_systems(render_scene_system);
        // builder.schedule_entry(RenderSchedule::Render).add_systems((
        //     render::render_loading_screen_system.run_if(in_state(AppState::Loading)),
        //     render::render_main_scene_system.run_if(in_state(AppState::Running)),
        // ));
    }
}
