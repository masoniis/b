use crate::prelude::*;
use bevy_ecs::{prelude::Component, system::Commands};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::task::JoinHandle;

/// A trait for any component that represents a long-running, async task.
pub trait AsyncTask<T>: Component {
    /// Polls the task and returns the handle inner type if finished.
    fn poll_result(&mut self) -> Option<T>;
}

/// A generic wrapper for a Tokio JoinHandle that provides polling.
pub struct TokioTask<T: Send + 'static> {
    pub handle: JoinHandle<T>,
}

impl<T: Send + 'static> TokioTask<T> {
    /// Creates a new task wrapper.
    pub fn new(handle: JoinHandle<T>) -> Self {
        Self { handle }
    }

    /// Polls the inner handle with a given error message.
    pub fn poll(&mut self, error_msg: &str) -> Option<T> {
        let waker = futures::task::noop_waker();
        let mut context = Context::from_waker(&waker);

        match Pin::new(&mut self.handle).poll(&mut context) {
            Poll::Ready(Ok(result)) => Some(result),
            Poll::Ready(Err(e)) => {
                error!("{}: {:?}", error_msg, e);
                None
            }
            Poll::Pending => None,
        }
    }
}

/// A command a task can return to execute ecs logic. Since it isn't
/// safe to edit the world asynchronously, this callback can be used.
pub type TaskResultCallback = Box<dyn FnOnce(&mut Commands) + Send>;

// INFO: ------------------------
//         Sim world task
// ------------------------------

/// Marks a loading task in the simulation world that returns a callback.
///
/// When all tasks return, a system can initiate a transition to a new
/// state.
#[derive(Component)]
pub struct SimulationWorldLoadingTaskComponent {
    pub task: TokioTask<TaskResultCallback>,
}

impl AsyncTask<TaskResultCallback> for SimulationWorldLoadingTaskComponent {
    fn poll_result(&mut self) -> Option<TaskResultCallback> {
        self.task.poll("Simulation loading task failed")
    }
}

/// Marks a loading task in the rendering world that returns nothing.
///
/// When all tasks return, a system can initiate a transition to a new
/// state.
#[derive(Component)]
pub struct RenderWorldLoadingTaskComponent {
    pub task: TokioTask<TaskResultCallback>,
}

impl AsyncTask<TaskResultCallback> for RenderWorldLoadingTaskComponent {
    fn poll_result(&mut self) -> Option<TaskResultCallback> {
        self.task.poll("Simulation loading task failed")
    }
}
