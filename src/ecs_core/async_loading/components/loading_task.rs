use bevy_ecs::{prelude::Component, system::Commands};
use bevy_tasks::Task;

// /// A trait for any component that represents a long-running, async loading task.
// pub trait AsyncTask: Component {
//     /// Returns true if the async task has completed.
//     fn is_finished(&self) -> bool;
// }

/// A command a task can return to execute ecs logic. Since it isn't
/// safe to edit the world asynchronously, this callback can be used.
pub type TaskResultCallback = Box<dyn FnOnce(&mut Commands) + Send>;

// INFO: ------------------------
//         Sim world task
// ------------------------------

#[derive(Component)]
pub struct SimulationWorldLoadingTaskComponent {
    pub task: Task<TaskResultCallback>,
}

#[derive(Component)]
pub struct RenderWorldLoadingTaskComponent {
    pub task: Task<TaskResultCallback>,
}
