use super::prepare::{PreparedUiBatches, UiBatch};
use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct RenderPhase<T: 'static> {
    pub queue: Vec<T>,
}

pub type UiPhaseItem = UiBatch;

pub fn queue_ui_system(
    mut ui_phase: ResMut<RenderPhase<UiPhaseItem>>,
    prepared_batches: Res<PreparedUiBatches>,
) {
    ui_phase.queue.clear();
    ui_phase
        .queue
        .extend(prepared_batches.batches.iter().copied());
}
