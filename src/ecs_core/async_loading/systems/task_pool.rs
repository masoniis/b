use bevy_ecs::system::NonSendMarker;
use bevy_tasks::{AsyncComputeTaskPool, TaskPoolBuilder};

/// System that ticks the global task pools.
///
/// Must run on the main thread.
pub fn tick_global_task_pools_system(_main_thread_marker: NonSendMarker) {
    AsyncComputeTaskPool::get().with_local_executor(|async_executor| {
        for _ in 0..100 {
            async_executor.try_tick();
        }
    });
}

/// A function to set up and initialize the global task pools.
pub fn setup_global_task_pools_system() {
    AsyncComputeTaskPool::get_or_init(|| {
        TaskPoolBuilder::default()
            .thread_name("Async Compute".to_string())
            .num_threads(std::thread::available_parallelism().unwrap().get())
            .build()
    });
}
