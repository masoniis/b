use crate::{
    ecs_modules::{
        input::InputModulePlugin,
        player::PlayerModulePlugin,
        rendering::{CameraUniformResource, RenderQueueResource, RenderingModulePlugin},
        screen_text::ScreenTextModulePlugin,
        state_machine::{
            resources::{AppState, CurrentState, GameState, NextState, PrevState},
            StateMachineModulePlugin,
        },
        world::WorldModulePlugin,
    },
    ecs_modules::{Plugin, Schedules},
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
            .add_resource(PrevState {
                val: None::<AppState>,
            })
            .add_resource(PrevState {
                val: None::<GameState>,
            })
            .add_resource(CurrentState {
                val: AppState::default(),
            })
            .add_resource(CurrentState {
                val: GameState::default(),
            })
            .add_resource(NextState {
                val: None::<AppState>,
            })
            .add_resource(NextState {
                val: None::<GameState>,
            });

        builder
            .add_plugin(StateMachineModulePlugin)
            .add_plugin(ScreenTextModulePlugin)
            .add_plugin(RenderingModulePlugin)
            .add_plugin(PlayerModulePlugin)
            .add_plugin(InputModulePlugin)
            .add_plugin(WorldModulePlugin);

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

        for (_, schedule) in self.schedules.drain_dynamic_schedules() {
            self.world.add_schedule(schedule);
        }

        EcsState {
            world: self.world,
            schedules: self.schedules,
            render_state,
        }
    }
}
