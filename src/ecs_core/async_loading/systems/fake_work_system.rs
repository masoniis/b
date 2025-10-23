use crate::{
    ecs_core::async_loading::loading_task::{
        SimulationWorldLoadingTaskComponent, TaskResultCallback, TokioTask,
    },
    prelude::*,
};
use bevy_ecs::prelude::*;

#[instrument(skip_all)]
pub fn start_fake_work_system(mut commands: Commands) {
    info!("Spawning fake work task on a background thread...");

    const DURATION_SECS: u64 = 3;

    // Spawn the task
    let task_handle = tokio::spawn(async move {
        for i in 1..=DURATION_SECS {
            info!(
                "(Background Thread) Fake working... step {}/{}",
                i, DURATION_SECS
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        let callback: TaskResultCallback = Box::new(|_commands| {
            info!("(Callback) Finished performing fake work!");
        });

        callback
    });

    commands.spawn(SimulationWorldLoadingTaskComponent {
        task: TokioTask {
            handle: task_handle,
        },
    });
}
