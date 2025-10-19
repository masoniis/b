pub mod app_lifecycle;
pub mod block;
pub mod chunk;
pub mod global_resources;
pub mod input;
pub mod player;
pub mod scheduling;
pub mod time;
pub mod user_interface;

// INFO: -----------------------------
//         Sim world interface
// -----------------------------------

pub use self::scheduling::{OnExit, SimulationSchedule, SimulationSet};
use crate::render_world::{
    global_extract::utils::initialize_simulation_world_for_extract, textures::TextureRegistry,
};
use crate::simulation_world::block::BlockPlugin;
use crate::simulation_world::chunk::ChunkGenerationPlugin;
use crate::simulation_world::global_resources::MeshAsset;
use crate::simulation_world::input::InputModulePlugin;
use crate::simulation_world::player::PlayerModulePlugin;
use crate::simulation_world::scheduling::{FixedUpdateSet, StartupSet};
use crate::simulation_world::time::TimeControlPlugin;
use crate::{
    ecs_core::{worlds::SimulationWorldMarker, CommonEcsInterface, EcsBuilder, PluginGroup},
    simulation_world::app_lifecycle::AppLifecyclePlugin,
};
use bevy_ecs::prelude::*;
use global_resources::TextureMapResource;
use input::resources::WindowSizeResource;
use std::ops::{Deref, DerefMut};
use user_interface::UiPlugin;
use winit::window::Window;

pub struct SimulationWorldInterface {
    pub common: CommonEcsInterface,
}

impl SimulationWorldInterface {
    pub fn send_event<E: Event>(&mut self, event: E) {
        self.common.world.send_event(event);
    }
}

impl Deref for SimulationWorldInterface {
    type Target = CommonEcsInterface;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for SimulationWorldInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

impl SimulationWorldInterface {
    pub fn new(window: &Window, registry: TextureRegistry) -> Self {
        let mut builder = EcsBuilder::new();

        // add resources built from the app
        builder
            .add_resource(WindowSizeResource::new(window.inner_size()))
            .add_resource(TextureMapResource { registry });

        // configure schedule sets before adding plugins
        builder
            .schedules
            .entry(SimulationSchedule::Startup)
            .configure_sets((StartupSet::UserInterface, StartupSet::LoadingTasks).chain());

        builder
            .schedules
            .entry(SimulationSchedule::FixedUpdate)
            .configure_sets((FixedUpdateSet::PreUpdate, FixedUpdateSet::MainLogic).chain());

        builder
            .schedules
            .entry(SimulationSchedule::Main)
            .configure_sets(
                (
                    SimulationSet::Input,
                    SimulationSet::PreUpdate,
                    SimulationSet::Update,
                    SimulationSet::Physics,
                    SimulationSet::PostUpdate,
                    SimulationSet::RenderPrep,
                )
                    .chain(),
            );

        // now add plugins, which can safely use the configured sets
        builder
            .add_plugins(SharedPlugins)
            .add_plugins(ClientOnlyPlugins);

        return Self::build_simulation_world(builder);
    }

    fn build_simulation_world(mut builder: EcsBuilder) -> SimulationWorldInterface {
        for (_, schedule) in builder.schedules.drain_schedules() {
            builder.world.add_schedule(schedule);
        }

        let mut interface = SimulationWorldInterface {
            common: CommonEcsInterface {
                world: builder.world,
            },
        };

        initialize_simulation_world_for_extract(&mut interface.common.world);

        interface
            .common
            .world
            .insert_resource(SimulationWorldMarker);

        return interface;
    }
}

// INFO: ---------------------------------
//         Plugin Groups (private)
// ---------------------------------------

/// Plugins to run on both the server and client
struct SharedPlugins;
impl PluginGroup for SharedPlugins {
    fn build(self, builder: &mut EcsBuilder) {
        builder
            .add_plugin(AppLifecyclePlugin)
            .add_plugin(BlockPlugin)
            .add_plugin(ChunkGenerationPlugin)
            .add_plugin(TimeControlPlugin)
            .add_plugin(PlayerModulePlugin);
    }
}

/// Plugins to run on solely on a client (UI, etc)
struct ClientOnlyPlugins;
impl PluginGroup for ClientOnlyPlugins {
    fn build(self, builder: &mut EcsBuilder) {
        builder
            .add_resource(global_resources::camera::CameraResource::default())
            .add_resource(global_resources::asset_storage::AssetStorageResource::<
                MeshAsset,
            >::default())
            .add_plugin(UiPlugin)
            .add_plugin(InputModulePlugin);
    }
}
