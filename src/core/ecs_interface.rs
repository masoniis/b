use crate::{
    ecs_modules::{
        graphics::RenderingModulePlugin,
        input::InputModulePlugin,
        player::PlayerModulePlugin,
        schedules::ScheduleLables,
        screen_text::ScreenTextModulePlugin,
        state_machine::{
            resources::{AppState, CurrentState, GameState, NextState, PrevState},
            StateMachineModulePlugin,
        },
        world::WorldModulePlugin,
        Plugin, Schedules,
    },
    ecs_resources::{
        asset_storage::MeshAsset, time::TimeResource, AssetStorageResource, CameraResource,
    },
    prelude::*,
};
use bevy_ecs::prelude::{Event, Resource, World};

/// An interface for the app to safely interact with the ECS world
pub struct ExternalEcsInterface {
    world: World,
}

impl ExternalEcsInterface {
    // Send an event to the ECS world
    pub fn send_event<E: Event>(&mut self, event: E) {
        self.world.send_event(event);
    }

    // Get the value of the AppState resource in the world
    pub fn get_app_state(&self) -> AppState {
        self.world
            .get_resource::<CurrentState<AppState>>()
            .unwrap()
            .val
            .clone()
    }

    pub fn run_schedule(&mut self, label: ScheduleLables) {
        match self.world.try_run_schedule(label.clone()) {
            Ok(_) => {}
            Err(error) => {
                warn!(
                    "Schedule with label {:?} not found or failed to run: {}",
                    label, error
                );
            }
        }
    }
}

pub struct ExternalEcsInterfaceBuilder {
    world: World,
    schedules: Schedules,
}

impl ExternalEcsInterfaceBuilder {
    pub fn default() -> ExternalEcsInterfaceBuilder {
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

    pub fn build(self) -> ExternalEcsInterface {
        // We take ownership of world and schedules, add the schedules
        // necessary to the world, and then give ownership of world to
        // the EcsState struct.
        let mut schedules = self.schedules;
        let mut world = self.world;

        for (_, schedule) in schedules.drain_dynamic_schedules() {
            world.add_schedule(schedule);
        }

        world.add_schedule(schedules.startup);
        world.add_schedule(schedules.loading);
        world.add_schedule(schedules.main);

        ExternalEcsInterface { world }
    }
}
