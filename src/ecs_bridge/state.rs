use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::{
        rendering::{CameraUniformResource, RenderQueueResource},
        InputModuleBuilder, PlayerModuleBuilder, RenderingModuleBuilder, ScreenTextModuleBuilder,
        WorldModuleBuilder,
    },
    ecs_resources::{
        asset_storage::MeshAsset, time::TimeResource, AssetStorageResource, CameraResource,
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

pub struct EcsState {
    pub world: World,
    pub schedules: Schedules,
    pub render_state: SystemState<(
        ResMut<'static, RenderQueueResource>,
        Res<'static, AssetStorageResource<MeshAsset>>,
        Res<'static, CameraUniformResource>,
    )>,
}

pub struct EcsStateBuilder {
    world: World,
    schedules: Schedules,
}

impl EcsState {
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

    pub fn run_startup(&mut self) {
        if !self.world.contains_resource::<WindowResource>() {
            panic!("WindowResource must be added to the world before running startup systems.");
        }
        self.schedules.startup.run(&mut self.world);
    }

    pub fn run_main(&mut self) {
        self.schedules.input.run(&mut self.world);
        self.schedules.main.run(&mut self.world);
    }
}

impl EcsStateBuilder {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert_resource(TimeResource::default());
        world.insert_resource(CameraResource::default());
        world.insert_resource(AssetStorageResource::<MeshAsset>::default());

        Self {
            world,
            schedules: Schedules::new(),
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(&mut self.schedules, &mut self.world);
        self
    }

    pub fn build(mut self) -> EcsState {
        let render_state = SystemState::new(&mut self.world);
        EcsState {
            world: self.world,
            schedules: self.schedules,
            render_state,
        }
    }
}
