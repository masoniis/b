use crate::{
    ecs_core::async_loading::loading_task::SimulationWorldLoadingTaskComponent, prelude::*,
};
use bevy_ecs::prelude::*;

#[instrument(skip_all)]
pub fn start_fake_work_system(mut commands: Commands) {
    info!("Spawning fake work task on a background thread...");

    // Spawn the task
    let task_handle = tokio::spawn(async move {
        for i in 1..=5 {
            info!("(Background Thread) Fake working... step {}/2", i);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
        info!("(Background Thread) Fake work finished!");
    });

    commands.spawn(SimulationWorldLoadingTaskComponent {
        handle: task_handle,
    });
}
