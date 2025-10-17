pub mod global_extract;
pub mod graphics_context;
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
use crate::ecs_core::worlds::RenderWorldMarker;
use crate::prelude::*;
use crate::render_world::global_extract::{
    simulation_world_resource_changed, ExtractComponentPlugin, RenderWindowSizeResource,
};
use crate::render_world::graphics_context::{reconfigure_wgpu_surface_system, GraphicsContext};
use crate::render_world::passes::ui_pass::RenderUiPlugin;
use crate::render_world::scheduling::{RenderSchedule, RenderSet};
use crate::render_world::textures::GpuTextureArray;
use crate::simulation_world::chunk::MeshComponent;
use crate::simulation_world::input::resources::WindowSizeResource;
use crate::{
    ecs_core::state_machine::{self, in_state, StatePlugin},
    render_world::{
        global_extract::{RenderCameraResource, RenderMeshStorageResource, RenderTimeResource},
        passes::opaque_pass::{
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
use bevy_ecs::schedule::common_conditions::resource_changed_or_removed;
use bevy_ecs::schedule::IntoScheduleConfigs;
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
    pub fn new(graphics_context: GraphicsContext, texture_array: GpuTextureArray) -> Self {
        let mut builder = EcsBuilder::new();

        // TODO: Texture could have it's own module that depends on graphics context instead of
        // hardcoding it here in the interface potentially

        // INFO: -----------------------------------------------------
        //         Set up graphics-context dependent resources
        // -----------------------------------------------------------

        // Setup render graph runs as an early system since it needs mutable world access
        setup_render_graph(&mut builder.world);

        // Add any resources that require specific app input
        builder
            // GraphicsContextResource is a resource initialized here because
            // it requires the graphics_context passed directly from the app.
            .add_resource(GraphicsContextResource {
                context: graphics_context,
            })
            .add_resource(TextureArrayResource {
                array: texture_array,
            })
            .add_resource(RenderWorldMarker);

        // INFO: --------------------------------
        //         Non-mod specific setup
        // --------------------------------------

        builder
            .schedules
            .entry(RenderSchedule::Main)
            .configure_sets((RenderSet::Prepare, RenderSet::Queue, RenderSet::Render).chain());

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
                    global_extract::clone_resource_system::<AssetStorageResource<MeshAsset>>,
                    global_extract::extract_resource_system::<RenderTimeResource>,
                    (global_extract::extract_resource_system::<RenderWindowSizeResource>)
                        .run_if(simulation_world_resource_changed::<WindowSizeResource>),
                    global_extract::extract_state_system::<GameState>,
                    global_extract::extract_state_system::<AppState>,
                ),
                (global_extract::extract_resource_system::<RenderCameraResource>)
                    .run_if(in_state(AppState::Running)),
            ));

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            (
                reconfigure_wgpu_surface_system
                    .run_if(resource_changed_or_removed::<RenderWindowSizeResource>),
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
