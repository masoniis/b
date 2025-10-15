use bevy_ecs::prelude::Component;
use tokio::task::JoinHandle;

/// A trait for any component that represents a long-running, async loading task.
pub trait LoadingTask: Component {
    /// Returns true if the async task has completed.
    fn is_finished(&self) -> bool;
}

// INFO: ------------------------
//         Sim world task
// ------------------------------

/// Marks a loading task in the simulation world that returns nothing.
///
/// When all tasks return, a system can initiate a transition to a new
/// state.
#[derive(Component)]
pub struct SimulationWorldLoadingTaskComponent {
    pub handle: JoinHandle<()>,
}

impl LoadingTask for SimulationWorldLoadingTaskComponent {
    fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}

/// Marks a loading task in the rendering world that returns nothing.
///
/// When all tasks return, a system can initiate a transition to a new
/// state.
#[derive(Component)]
pub struct RenderWorldLoadingTaskComponent {
    pub handle: JoinHandle<()>,
}

impl LoadingTask for RenderWorldLoadingTaskComponent {
    fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}
