use crate::prelude::*;
use bevy_ecs::{prelude::Component, system::Commands};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::task::JoinHandle;

/// A trait for any component that represents a long-running, async loading task.
pub trait LoadingTask: Component {
    /// Polls the task and returns the callback if finished.
    fn poll_result(&mut self) -> Option<TaskResultCallback>;
}

/// A command a task can return to execute ecs logic. Since it isn't
/// safe to edit the world asynchronously, this callback can be used.
pub type TaskResultCallback = Box<dyn FnOnce(&mut Commands) + Send>;

// INFO: ------------------------
//         Sim world task
// ------------------------------

/// Marks a loading task in the simulation world that returns nothing.
///
/// When all tasks return, a system can initiate a transition to a new
/// state.
#[derive(Component)]
pub struct SimulationWorldLoadingTaskComponent {
    pub handle: JoinHandle<TaskResultCallback>,
}

impl LoadingTask for SimulationWorldLoadingTaskComponent {
    fn poll_result(&mut self) -> Option<TaskResultCallback> {
        // Create a no-op waker since we're just checking if ready
        let waker = futures::task::noop_waker();
        let mut context = Context::from_waker(&waker);

        // Pin and poll the handle (Future trait must be in scope)
        match Pin::new(&mut self.handle).poll(&mut context) {
            Poll::Ready(Ok(callback)) => {
                // Task finished successfully
                Some(callback)
            }
            Poll::Ready(Err(e)) => {
                // Task panicked or was cancelled
                error!("Simulation loading task failed: {:?}", e);
                None
            }
            Poll::Pending => {
                // Task is not ready yet
                None
            }
        }
    }
}

/// Marks a loading task in the rendering world that returns nothing.
///
/// When all tasks return, a system can initiate a transition to a new
/// state.
#[derive(Component)]
pub struct RenderWorldLoadingTaskComponent {
    pub handle: JoinHandle<TaskResultCallback>,
}

impl LoadingTask for RenderWorldLoadingTaskComponent {
    fn poll_result(&mut self) -> Option<TaskResultCallback> {
        let waker = futures::task::noop_waker();
        let mut context = Context::from_waker(&waker);

        match Pin::new(&mut self.handle).poll(&mut context) {
            Poll::Ready(Ok(callback)) => Some(callback),
            Poll::Ready(Err(e)) => {
                error!("Render loading task failed: {:?}", e);
                None
            }
            Poll::Pending => None,
        }
    }
}
