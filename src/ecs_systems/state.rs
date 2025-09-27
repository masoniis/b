use crate::{
    ecs_modules::{
        camera_control_system, changed_mesh_system, changed_screen_text_system,
        chunk_generation_system, init_screen_diagnostics_system, removed_mesh_system,
        removed_screen_text_system, screen_diagnostics_system, time_system,
    },
    ecs_resources::{
        asset_storage::MeshAsset, input::InputResource, time::TimeResource, AssetStorageResource,
        CameraResource, CameraUniformResource, RenderQueueResource, WindowResource,
    },
};
use bevy_ecs::{
    prelude::*,
    schedule::{Schedule, ScheduleLabel},
    system::SystemState,
    world::World,
};

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Schedules {
    Startup,
    Main,
}

/// A container for the entire ECS World, its schedules, and cached system states.
pub struct EcsState {
    pub world: World,
    pub startup_scheduler: Schedule,
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
        // WindowResource will be added later when the window is created

        // Build the startup schedule
        let mut startup_scheduler = Schedule::new(Schedules::Startup);
        startup_scheduler.add_systems((
            chunk_generation_system,
            init_screen_diagnostics_system,
            // mesh_render_system.after(chunk_generation_system),
        ));

        // Build the main schedule
        let mut main_scheduler = Schedule::new(Schedules::Main);
        main_scheduler.add_systems((
            time_system.before(screen_diagnostics_system),
            screen_diagnostics_system,
            camera_control_system,
            changed_screen_text_system.after(screen_diagnostics_system),
            removed_screen_text_system.after(screen_diagnostics_system),
            changed_mesh_system,
            removed_mesh_system,
        ));

        // Create a cached SystemState for efficient access to render data
        let render_state = SystemState::new(&mut world);

        Self {
            world,
            startup_scheduler,
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
        self.main_scheduler.run(&mut self.world);
    }
}
