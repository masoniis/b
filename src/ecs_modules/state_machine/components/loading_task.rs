use bevy_ecs::prelude::Component;
use tokio::task::JoinHandle;

/// Marks a loading task that returns nothing.
///
/// In theory, when all LoadingTasks return, the system
/// can safely transition to the next (non-loading) state.
#[derive(Component)]
pub struct LoadingTaskComponent {
    pub handle: JoinHandle<()>,
}
