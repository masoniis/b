pub mod context;
pub mod extract;
pub mod passes;
pub mod resources;
pub mod scheduling;
pub mod textures;
pub mod types;
pub mod uniforms;

// INFO: --------------------------------
//         Render world interface
// --------------------------------------

use crate::ecs_core::async_loading::poll_render_loading_tasks;
use crate::ecs_core::state_machine::{AppState, GameState};
use crate::prelude::*;
use crate::render_world::extract::{
    simulation_world_resource_changed, ExtractComponentPlugin, RenderWindowSizeResource,
};
use crate::render_world::passes::ui_pass::RenderUiPlugin;
use crate::render_world::scheduling::{RenderSchedule, RenderSet};
use crate::simulation_world::chunk::MeshComponent;
use crate::simulation_world::input::resources::WindowSizeResource;
use crate::{
    ecs_core::state_machine::{self, in_state, StatePlugin},
    render_world::{
        extract::{RenderCameraResource, RenderMeshStorageResource, RenderTimeResource},
        passes::main_pass::{
            prepare::{
                self, MainTextureBindGroup, MeshPipelineLayoutsResource, ModelBindGroup,
                ViewBindGroup,
            },
            queue::{self, Opaque3dRenderPhase},
        },
        resources::PipelineCacheResource,
    },
    simulation_world::global_resources::{AssetStorageResource, MeshAsset},
};
use crate::{ecs_core::worlds::RenderWorldMarker, render_world::textures::load_texture_array};
use bevy_ecs::schedule::IntoScheduleConfigs;
use context::GraphicsContext;
use passes::setup_render_graph;
use resources::{GraphicsContextResource, TextureArrayResource};
use std::ops::{Deref, DerefMut};

pub struct RenderWorldInterface {
    pub common: CommonEcsInterface,
}

impl Deref for RenderWorldInterface {
    type Target = CommonEcsInterface;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for RenderWorldInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl RenderWorldInterface {
    /// Creates a new render world with a sane default configuration
    pub fn new(graphics_context: GraphicsContext) -> Self {
        let mut builder = EcsBuilder::new();

        builder
            .schedules
            .entry(RenderSchedule::Main)
            .configure_sets((RenderSet::Prepare, RenderSet::Queue, RenderSet::Render).chain());

        // As a root resource that requites input from app, graphics context must be
        // inserted before we do any other system building.
        //
        // This is because systems can't create it themselves like most other resources.
        let (texture_array, _texture_registry) =
            load_texture_array(&graphics_context.device, &graphics_context.queue).unwrap();

        // Setup render graph runs as an early system since it needs mutable world access
        setup_render_graph(&mut builder.world);

        // Add any resources that require specific app input
        builder
            .add_resource(TextureArrayResource {
                array: texture_array,
            })
            .add_resource(GraphicsContextResource {
                context: graphics_context,
            })
            .add_resource(RenderWorldMarker);

        // INFO: --------------------------------
        //         Non-mod specific setup
        // --------------------------------------

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

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(poll_render_loading_tasks.run_if(in_state(AppState::StartingUp)));

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

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            (
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
            )
                .in_set(RenderSet::Prepare),
        );

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(queue::queue_mesh_system.in_set(RenderSet::Queue));

        builder
            .schedule_entry(RenderSchedule::Main)
            .add_systems(passes::render_graph::render_graph_system.in_set(RenderSet::Queue));

        return Self::build_render_world(builder);
    }

    /// Builds the final state and returns the final render world interface.
    fn build_render_world(mut builder: EcsBuilder) -> RenderWorldInterface {
        for (_, schedule) in builder.schedules.drain_schedules() {
            builder.world.add_schedule(schedule);
        }

        RenderWorldInterface {
            common: CommonEcsInterface {
                world: builder.world,
            },
        }
    }
}
