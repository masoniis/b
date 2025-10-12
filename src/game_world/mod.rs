use crate::{
    ecs_core::{state_machine::resources::CurrentState, worlds::GameWorldMarker},
    game_world::{
        global_resources::MeshAsset, input::InputModulePlugin, player::PlayerModulePlugin,
        schedules::GameSchedule, screen_text::ScreenTextModulePlugin, world::WorldModulePlugin,
    },
    prelude::*,
    render_world::{extract::utils::initialize_main_world_for_extract, textures::TextureRegistry},
};
use app_lifecycle::{AppLifecyclePlugin, AppState};
use bevy_ecs::prelude::*;
use global_resources::TextureMapResource;
use input::resources::WindowSizeResource;
use std::ops::{Deref, DerefMut};
use ui::UiPlugin;
use winit::window::Window;

pub mod app_lifecycle;
pub mod global_resources;
pub mod graphics_old;
pub mod input;
pub mod player;
pub mod schedules;
pub mod screen_text;
pub mod system_sets;
pub mod ui;
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

pub fn configure_game_world(registry: TextureRegistry, window: &Window) -> EcsBuilder {
    let mut builder = EcsBuilder::new();

    // Add resources built from the app
    builder
        .add_resource(WindowSizeResource::new(window.inner_size()))
        .add_resource(TextureMapResource { registry });

    // Configure core schedule sets before adding plugins
    builder.schedules.entry(GameSchedule::Main).configure_sets(
        (
            GameSet::Input,
            GameSet::PreUpdate,
            GameSet::Update,
            GameSet::Physics,
            GameSet::PostUpdate,
            GameSet::RenderPrep,
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

    let mut interface = GameWorldInterface {
        common: CommonEcsInterface {
            world: builder.world,
        },
    };

    initialize_main_world_for_extract(&mut interface.common.world);

    interface.common.world.insert_resource(GameWorldMarker);

    return interface;
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
            .add_plugin(UiPlugin)
            .add_plugin(InputModulePlugin);
    }
}
