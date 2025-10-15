use crate::ecs_core::async_loading::LoadingTracker;
use crate::prelude::*;
use bevy_ecs::prelude::*;

pub fn reset_loading_tracker_system(mut loading_tracker: NonSendMut<LoadingTracker>) {
    info!("Resetting loading tracker...");
    loading_tracker.reset();
}
