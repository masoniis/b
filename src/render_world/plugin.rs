use super::extract::{
    simulation_world_resource_changed, ExtractComponentPlugin, RenderWindowSizeResource,
};
use super::passes;
use super::passes::ui_pass::RenderUiPlugin;
use crate::prelude::*;
use crate::simulation_world::chunk::MeshComponent;
use crate::simulation_world::input::resources::WindowSizeResource;
use crate::{
    ecs_core::state_machine::{self, in_state, StatePlugin},
    render_world::{
        extract::{self, RenderCameraResource, RenderMeshStorageResource, RenderTimeResource},
        passes::main_pass::{
            prepare::{
                self, MainTextureBindGroup, MeshPipelineLayoutsResource, ModelBindGroup,
                ViewBindGroup,
            },
            queue::{self, Opaque3dRenderPhase},
        },
        resources::PipelineCacheResource,
        RenderSchedule,
    },
    simulation_world::{
        app_lifecycle::{AppState, GameState},
        global_resources::{AssetStorageResource, MeshAsset},
    },
};
use bevy_ecs::prelude::*;

/// The main render world plugin that orchestrates all the phases of rendering
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, builder: &mut EcsBuilder) {
        // Resources for rendering
        builder
            .init_resource::<RenderTimeResource>()
            .init_resource::<RenderCameraResource>()
            .init_resource::<RenderMeshStorageResource>()
            .init_resource::<MeshPipelineLayoutsResource>()
            .init_resource::<Opaque3dRenderPhase>()
            .init_resource::<PipelineCacheResource>()
            .init_resource::<ViewBindGroup>()
            .init_resource::<MainTextureBindGroup>()
            .init_resource::<ModelBindGroup>();

        // Specifically implemented plugins
        builder.add_plugin(RenderUiPlugin);
        // Generic auto-constructed plugins
        builder.add_plugin(StatePlugin::<AppState>::default());
        builder.add_plugin(StatePlugin::<GameState>::default());
        builder.add_plugin(ExtractComponentPlugin::<MeshComponent>::default());

        // System builders
        builder
            .schedule_entry(RenderSchedule::Extract)
            .add_systems((
                (
                    extract::clone_resource_system::<AssetStorageResource<MeshAsset>>,
                    extract::extract_resource_system::<RenderTimeResource>,
                    (extract::extract_resource_system::<RenderWindowSizeResource>)
                        .run_if(simulation_world_resource_changed::<WindowSizeResource>),
                    extract::extract_state_system::<GameState>,
                    extract::extract_state_system::<AppState>,
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
                    // apply any state transitions detected during Extract phase
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
            .add_systems(passes::render_graph::render_graph_system);
    }
}
