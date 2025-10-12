use super::prepare::{PreparedUiBatches, UiRenderBatch};
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct RenderPhase<T: 'static + Send + Sync> {
    pub queue: Vec<T>,
}

impl<T: 'static + Send + Sync> Default for RenderPhase<T> {
    fn default() -> Self {
        Self { queue: Vec::new() }
    }
}

pub type UiPhaseItem = UiRenderBatch;

pub fn queue_ui_system(
    mut ui_phase: ResMut<RenderPhase<UiPhaseItem>>,
    prepared_batches: Res<PreparedUiBatches>,
) {
    ui_phase.queue.clear();
    ui_phase
        .queue
        .extend(prepared_batches.batches.iter().cloned());
}
