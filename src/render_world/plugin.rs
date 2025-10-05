use super::{
    extract::{
        extract_meshes::MeshEntityMap, RenderCameraResource, RenderMeshStorageResource,
        RenderTimeResource,
    },
    prepare::{
        resources::bind_groups::ModelBindGroup, resources::LoadingScreenPipelineLayoutsResource,
        MainTextureBindGroup, PipelineCacheResource, ViewBindGroup,
    },
    queue::Opaque3dRenderPhase,
    render::render_scene_system,
    RenderSchedule,
};
use crate::{
    ecs_core::state_machine::{self, in_state, StatePlugin},
    game_world::{
        app_lifecycle::{AppState, GameState},
        global_resources::{AssetStorageResource, MeshAsset},
    },
    prelude::*,
    render_world::{
        extract::{self},
        prepare::{self, MeshPipelineLayoutsResource},
        queue::{self, RenderQueueResource},
    },
};
use bevy_ecs::prelude::*;

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
        builder.init_resource::<Opaque3dRenderPhase>();
        builder.init_resource::<PipelineCacheResource>();
        builder.init_resource::<LoadingScreenPipelineLayoutsResource>();
        builder.init_resource::<ViewBindGroup>();
        builder.init_resource::<MainTextureBindGroup>();
        builder.init_resource::<ModelBindGroup>();

        // Plugin dependencies
        builder.add_plugin(StatePlugin::<AppState>::default());
        builder.add_plugin(StatePlugin::<GameState>::default());

        // System builders
        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                (
                    extract::clone_resource_system::<AssetStorageResource<MeshAsset>>,
                    extract::extract_resource_system::<RenderTimeResource>,
                    extract::extract_state_system::<GameState>,
                    extract::extract_state_system::<AppState>,
                    extract::extract_meshes_system,
                ),
                (extract::extract_resource_system::<RenderCameraResource>)
                    .run_if(in_state(AppState::Running)),
            ));

        builder
            .schedule_entry(RenderSchedule::Prepare)
            .add_systems((
                (
                    prepare::prepare_render_buffers_system,
                    prepare::prepare_pipelines_system,
                    // Apply any state transitions detected during Extract phase
                    state_machine::apply_state_transition_system::<AppState>,
                    state_machine::apply_state_transition_system::<GameState>,
                ),
                (
                    prepare::prepare_view_bind_group_system,
                    prepare::prepare_meshes_system,
                )
                    .run_if(in_state(AppState::Running)),
            ));

        builder
            .schedule_entry(RenderSchedule::Queue)
            .add_systems(queue::queue_mesh_system);

        builder
            .schedule_entry(RenderSchedule::Render)
            .add_systems(render_scene_system.run_if(in_state(AppState::Running)));
        // builder.schedule_entry(RenderSchedule::Render).add_systems((
        //     render::render_loading_screen_system.run_if(in_state(AppState::Loading)),
        //     render::render_main_scene_system.run_if(in_state(AppState::Running)),
        // ));
    }
}
