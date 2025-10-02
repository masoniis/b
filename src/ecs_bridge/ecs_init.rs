use crate::{
    ecs_bridge::{Plugin, Schedules},
    ecs_modules::{
        rendering::{CameraUniformResource, RenderQueueResource},
        state_machine::{
            resources::{AppState, CurrentState, GameState, NextState},
            StateMachineModuleBuilder,
        },
        InputModuleBuilder, PlayerModuleBuilder, RenderingModuleBuilder, ScreenTextModuleBuilder,
        WorldModuleBuilder,
    },
    ecs_resources::{
        asset_storage::MeshAsset, time::TimeResource, AssetStorageResource, CameraResource,
    },
};
use bevy_ecs::{
    prelude::*, resource::Resource, schedule::ScheduleLabel, system::SystemState, world::World,
};

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

impl EcsStateBuilder {
    pub fn default() -> EcsStateBuilder {
        let mut builder = Self::new();

        builder
            .add_resource(TimeResource::default())
            .add_resource(CameraResource::default())
            .add_resource(AssetStorageResource::<MeshAsset>::default())
            .add_resource(CurrentState {
                value: AppState::default(),
            })
            .add_resource(CurrentState {
                value: GameState::default(),
            })
            .add_resource(NextState {
                value: None::<AppState>,
            })
            .add_resource(NextState {
                value: None::<GameState>,
            });

        builder
            .add_plugin(InputModuleBuilder)
            .add_plugin(RenderingModuleBuilder)
            .add_plugin(ScreenTextModuleBuilder)
            .add_plugin(WorldModuleBuilder)
            .add_plugin(StateMachineModuleBuilder)
            .add_plugin(PlayerModuleBuilder);

        return builder;
    }

    pub fn new() -> Self {
        Self {
            world: World::new(),
            schedules: Schedules::new(),
        }
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        plugin.build(&mut self.schedules, &mut self.world);
        self
    }

    pub fn add_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
        self.world.insert_resource(resource);
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
