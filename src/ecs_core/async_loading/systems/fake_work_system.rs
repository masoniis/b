use crate::{
    ecs_core::async_loading::{
        load_tracking::LoadingTaskTracker,
        loading_task::{SimulationWorldLoadingTaskComponent, TaskResultCallback},
    },
    prelude::*,
};
use bevy_ecs::prelude::*;
use bevy_ecs::system::Commands;
use bevy_tasks::AsyncComputeTaskPool;
use futures_timer::Delay;
use rand::Rng;
use std::time::Duration;

#[instrument(skip_all)]
pub fn start_fake_work_system(mut commands: Commands, mut tracker: ResMut<LoadingTaskTracker>) {
    let task_pool = AsyncComputeTaskPool::get();

    tracker.register_spawn();

    let entity = commands.spawn_empty().id();
    let task = task_pool.spawn(async move {
        const WORK_DURATION: u64 = 3;
        for i in 1..=WORK_DURATION {
            info!(
                "[BACKGROUND {}] Fake working... step {}/{}",
                entity, i, WORK_DURATION
            );

            // do a lot of math for a long time
            let duration = Duration::from_secs_f32(rand::rng().random_range(0.05..1.0));
            Delay::new(duration).await;
        }
        info!("[BACKGROUND {}] Fake work finished!", entity);

        let callback: TaskResultCallback = Box::new(|_: &mut Commands| {
            info!("[CALLBACK] Task callback executed on main thread!");
        });

        callback
    });

    commands
        .entity(entity)
        .insert(SimulationWorldLoadingTaskComponent { task });
}
