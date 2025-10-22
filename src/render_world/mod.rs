pub mod global_extract;
pub mod graphics_context;
pub mod passes;
pub mod resources;
pub mod scheduling;
pub mod textures;
pub mod types;

// INFO: --------------------------------
//         Render world interface
// --------------------------------------

use crate::ecs_core::async_loading::poll_render_loading_tasks;
use crate::ecs_core::state_machine::{AppState, GameState};
use crate::ecs_core::worlds::RenderWorldMarker;
use crate::prelude::*;
use crate::render_world::global_extract::{
    extract_active_camera_system, simulation_world_resource_changed, ExtractComponentPlugin,
    RenderWindowSizeResource,
};
use crate::render_world::graphics_context::{GraphicsContext, GraphicsContextPlugin};
use crate::render_world::passes::core::setup_render_graph;
use crate::render_world::passes::RenderPassManagerPlugin;
use crate::render_world::scheduling::{RenderSchedule, RenderSet};
use crate::render_world::textures::GpuTextureArray;
use crate::simulation_world::chunk::MeshComponent;
use crate::simulation_world::input::resources::WindowSizeResource;
use crate::{
    ecs_core::state_machine::{self, in_state, StatePlugin},
    render_world::{
        global_extract::{RenderCameraResource, RenderMeshStorageResource, RenderTimeResource},
        resources::PipelineCacheResource,
    },
    simulation_world::asset_management::{AssetStorageResource, MeshAsset},
};
use bevy_ecs::schedule::IntoScheduleConfigs;
use resources::TextureArrayResource;
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
            .configure_sets(
                (
                    RenderSet::StateUpdate,
                    RenderSet::Prepare,
                    RenderSet::Queue,
                    RenderSet::Render,
                )
                    .chain(),
            );

        // Resources for rendering
        builder
            .init_resource::<RenderTimeResource>()
            .init_resource::<RenderCameraResource>()
            .init_resource::<RenderMeshStorageResource>()
            .init_resource::<PipelineCacheResource>();

        // Specifically implemented plugins
        builder
            .add_plugin(GraphicsContextPlugin::new(graphics_context))
            .add_plugin(RenderPassManagerPlugin);
        // Generic auto-constructed plugins
        builder
            .add_plugin(StatePlugin::<AppState>::default())
            .add_plugin(StatePlugin::<GameState>::default())
            .add_plugin(ExtractComponentPlugin::<MeshComponent>::default());

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
                extract_active_camera_system,
            ));

        builder.schedule_entry(RenderSchedule::Main).add_systems(
            // apply any state transitions detected during Extract phase
            (
                state_machine::apply_state_transition_system::<AppState>,
                state_machine::apply_state_transition_system::<GameState>,
            )
                .in_set(RenderSet::StateUpdate),
        );

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
