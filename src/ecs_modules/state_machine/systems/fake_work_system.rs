use crate::ecs_modules::state_machine::LoadingTaskComponent;
use crate::prelude::*;
use bevy_ecs::prelude::*;

pub fn start_fake_work_system(mut commands: Commands) {
    info!("Spawning fake work task on a background thread...");

    // Spawn the task. This closure will be executed on a background thread.
    let task_handle = tokio::spawn(async move {
        for i in 1..=2 {
            info!("(Background Tokio Thread) Working... step {}/2", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        info!("(Background Tokio Thread) Work finished!");
    });

    commands.spawn(LoadingTaskComponent {
        handle: task_handle,
    });
}
