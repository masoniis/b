use bevy_ecs::prelude::*;

use super::prepare::PreparedUiNodes;

#[derive(Resource, Default)]
pub struct RenderPhase<T: 'static> {
    pub queue: Vec<T>,
}

#[derive(Default)]
pub struct UiPhaseItem {
    pub prepared_node_index: usize,
}

pub fn queue_ui_system(
    mut ui_phase: ResMut<RenderPhase<UiPhaseItem>>,
    prepared_nodes: Res<PreparedUiNodes>,
) {
    ui_phase.queue.clear();

    for (index, _) in prepared_nodes.nodes.iter().enumerate() {
        ui_phase.queue.push(UiPhaseItem {
            prepared_node_index: index,
        });
    }
}
