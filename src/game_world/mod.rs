use crate::{
    game_world::global_resources::MeshAsset,
    game_world::{
        input::InputModulePlugin,
        player::PlayerModulePlugin,
        schedules::GameSchedule,
        screen_text::ScreenTextModulePlugin,
        state_machine::resources::{AppState, CurrentState},
        world::WorldModulePlugin,
    },
    prelude::*,
};
use app_lifecycle::AppLifecyclePlugin;
use bevy_ecs::prelude::*;
use state_machine::{resources::GameState, StatePlugin};
use std::ops::{Deref, DerefMut};

pub mod app_lifecycle;
pub mod global_resources;
pub mod graphics;
pub mod input;
pub mod player;
pub mod schedules;
pub mod screen_text;
pub mod state_machine;
pub mod system_sets;
pub mod world;

// INFO: ------------------------------
//         Game world interface
// ------------------------------------

pub struct GameWorldInterface {
    pub common: CommonEcsInterface,
}

impl GameWorldInterface {
    pub fn send_event<E: Event>(&mut self, event: E) {
        self.common.world.send_event(event);
    }

    pub fn get_app_state(&self) -> AppState {
        self.common
            .world
            .get_resource::<CurrentState<AppState>>()
            .unwrap()
            .val
            .clone()
    }
}

impl Deref for GameWorldInterface {
    type Target = CommonEcsInterface;
    fn deref(&self) -> &Self::Target {
        &self.common
    }
}

impl DerefMut for GameWorldInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.common
    }
}

// INFO: ----------------------------
//         Game World Builder
// ----------------------------------

pub fn configure_game_world() -> EcsBuilder {
    let mut builder = EcsBuilder::new();

    // Configure core schedule sets before adding plugins
    builder.schedules.entry(GameSchedule::Main).configure_sets(
        (
            CoreSet::Input,
            CoreSet::PreUpdate,
            CoreSet::Update,
            CoreSet::Physics,
            CoreSet::PostUpdate,
            CoreSet::RenderPrep,
            CoreSet::Render,
        )
            .chain(),
    );

    // Now add plugins, which can safely use the configured sets
    builder
        .add_plugins(SharedPlugins)
        .add_plugins(ClientOnlyPlugins);

    builder
}

pub fn build_game_world(mut builder: EcsBuilder) -> GameWorldInterface {
    for (_, schedule) in builder.schedules.drain_schedules() {
        builder.world.add_schedule(schedule);
    }

    GameWorldInterface {
        common: CommonEcsInterface {
            world: builder.world,
        },
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
            .add_resource(global_resources::time::TimeResource::default())
            .add_plugin(AppLifecyclePlugin)
            .add_plugin(StatePlugin::<AppState>::default())
            .add_plugin(StatePlugin::<GameState>::default())
            .add_plugin(WorldModulePlugin)
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
            .add_plugin(ScreenTextModulePlugin)
            .add_plugin(InputModulePlugin);
    }
}
