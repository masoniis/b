use crate::{
    ecs_modules::{self, PlayerModule, RenderingModule, ScreenTextModule, WorldModule},
    ecs_resources::{
        asset_storage::{MeshAsset, TextureAsset},
        events::{KeyboardInputEvent, MouseInputEvent, MouseScrollEvent},
        input::InputResource,
        time::TimeResource,
        AssetStorageResource, CameraResource, CameraUniformResource, RenderQueueResource,
        WindowResource,
    },
};
use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
    system::SystemState,
    world::World,
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScheduleLables {
    Startup,
    Input,
    Main,
}

/// A container for the entire ECS World, its schedules, and cached system states.
pub struct EcsState {
    pub world: World,
    pub startup_scheduler: Schedule,
    pub input_scheduler: Schedule,
    pub main_scheduler: Schedule,
    pub render_state: SystemState<(
        ResMut<'static, RenderQueueResource>,
        Res<'static, AssetStorageResource<MeshAsset>>,
        Res<'static, CameraUniformResource>,
    )>,
}

impl EcsState {
    pub fn new() -> Self {
        let mut world = World::new();

        // Register all resources
        world.insert_resource(InputResource::new());
        world.insert_resource(TimeResource::default());
        world.insert_resource(CameraResource::default());
        world.insert_resource(RenderQueueResource::default());
        world.insert_resource(CameraUniformResource::default());
        world.insert_resource(AssetStorageResource::<MeshAsset>::default());
        world.insert_resource(AssetStorageResource::<TextureAsset>::default());
        // WindowResource will be added later when the window is created

        // Register events
        world.init_resource::<Events<KeyboardInputEvent>>();
        world.init_resource::<Events<MouseInputEvent>>();
        world.init_resource::<Events<MouseScrollEvent>>();

        // Set up the schedulers
        let mut startup_scheduler = Schedule::new(ScheduleLables::Startup);
        let mut input_scheduler = Schedule::new(ScheduleLables::Input);
        let mut main_scheduler = Schedule::new(ScheduleLables::Main);

        input_scheduler.add_systems(
            ecs_modules::input::main_system::update_input::update_input_system
                .before(ecs_modules::input::main_system::event_handler::input_event_handler),
        );
        input_scheduler
            .add_systems(ecs_modules::input::main_system::event_handler::input_event_handler);

        // Add all the modules
        ScreenTextModule::build(&mut startup_scheduler, &mut main_scheduler, &mut world);
        PlayerModule::build(&mut startup_scheduler, &mut main_scheduler, &mut world);
        RenderingModule::build(&mut startup_scheduler, &mut main_scheduler, &mut world);
        WorldModule::build(&mut startup_scheduler, &mut main_scheduler, &mut world);

        // Create a cached SystemState for efficient access to render data
        let render_state = SystemState::new(&mut world);

        Self {
            world,
            startup_scheduler,
            input_scheduler,
            main_scheduler,
            render_state,
        }
    }

    /// Runs the startup schedule a single time.
    pub fn run_startup(&mut self) {
        if !self.world.contains_resource::<WindowResource>() {
            panic!("WindowResource must be added to the world before running startup systems.");
        }

        self.startup_scheduler.run(&mut self.world);
    }

    /// Runs the main schedule once.
    pub fn run_main(&mut self) {
        self.input_scheduler.run(&mut self.world);
        self.main_scheduler.run(&mut self.world);
    }
}
