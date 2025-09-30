use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::{
        InputModuleBuilder, PlayerModuleBuilder, RenderingModuleBuilder, ScreenTextModuleBuilder,
        WorldModuleBuilder,
    },
    ecs_resources::{
        asset_storage::{MeshAsset, TextureAsset},
        time::TimeResource,
        AssetStorageResource, CameraResource, CameraUniformResource, RenderQueueResource,
        WindowResource,
    },
};
use bevy_ecs::{prelude::*, schedule::ScheduleLabel, system::SystemState, world::World};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleLables {
    Startup,
    Input,
    Main,
}

/// A container for the entire ECS World, its schedules, and cached system states.
pub struct EcsState {
    pub world: World,
    pub schedules: Schedules,
    pub render_state: SystemState<(
        ResMut<'static, RenderQueueResource>,
        Res<'static, AssetStorageResource<MeshAsset>>,
        Res<'static, CameraUniformResource>,
    )>,
}

/// A builder for constructing the main EcsState in a clean, declarative way.
pub struct EcsStateBuilder {
    world: World,
    schedules: Schedules,
}

impl EcsState {
    /// Creates a fully configured ECS state.
    pub fn new() -> Self {
        let mut builder = EcsStateBuilder::new();
        builder
            .add_plugin(InputModuleBuilder)
            .add_plugin(ScreenTextModuleBuilder)
            .add_plugin(PlayerModuleBuilder)
            .add_plugin(RenderingModuleBuilder)
            .add_plugin(WorldModuleBuilder);

        builder.build()
    }

    /// Runs the startup schedule a single time.
    pub fn run_startup(&mut self) {
        if !self.world.contains_resource::<WindowResource>() {
            panic!("WindowResource must be added to the world before running startup systems.");
        }

        self.schedules.startup.run(&mut self.world);
    }

    /// Runs the main game loop schedules once.
    pub fn run_main(&mut self) {
        self.schedules.input.run(&mut self.world);
        self.schedules.main.run(&mut self.world);
    }
}

impl EcsStateBuilder {
    /// Creates a new builder and registers core resources.
    /// Plugins are expected to register their own specific resources and events.
    pub fn new() -> Self {
        let mut world = World::new();

        // Register CORE resources. More specific resources will be added by plugins.
        world.insert_resource(TimeResource::default());
        world.insert_resource(CameraResource::default());
        world.insert_resource(RenderQueueResource::default());
        world.insert_resource(CameraUniformResource::default());
        world.insert_resource(AssetStorageResource::<MeshAsset>::default());
        world.insert_resource(AssetStorageResource::<TextureAsset>::default());

        Self {
            world,
            schedules: Schedules::new(),
        }
    }

    /// Adds a plugin to the application.
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(&mut self.schedules, &mut self.world);
        self // for method chaining
    }

    /// Consumes the builder and returns the final, fully constructed EcsState.
    pub fn build(mut self) -> EcsState {
        // cached SystemState for efficient access to render data
        let render_state = SystemState::new(&mut self.world);

        EcsState {
            world: self.world,
            schedules: self.schedules,
            render_state,
        }
    }
}
