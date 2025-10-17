use crate::ecs_core::async_loading::LoadingTracker;
use crate::prelude::*;
use bevy_ecs::prelude::*;

#[instrument(skip_all)]
pub fn reset_loading_tracker_system(loading_tracker: Res<LoadingTracker>) {
    info!("Resetting loading tracker...");
    loading_tracker.reset();
}
