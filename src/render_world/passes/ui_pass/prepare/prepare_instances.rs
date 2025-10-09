use crate::{
    game_world::ui::components::UiBackground,
    prelude::*,
    render_world::{
        extract::{extract_component::ExtractedItems, ui::UiNodeExtractor},
        passes::ui_pass::startup::{UiInstanceBuffer, UiInstanceData},
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;

/// A resource to store all prepared UI nodes for this frame.
#[derive(Resource, Default)]
pub struct PreparedUiNodes {
    pub nodes: Vec<PreparedUiNode>,
}

// The fully prepared GPU data for a single UI node.
pub struct PreparedUiNode {
    pub dynamic_offset: u32,
}

pub fn prepare_ui_instances_system(
    gfx: Res<GraphicsContextResource>,
    extracted_nodes: Res<ExtractedItems<UiNodeExtractor>>,
    mut instance_buffer: ResMut<UiInstanceBuffer>,
    mut prepared_nodes: ResMut<PreparedUiNodes>,
) {
    instance_buffer.instances.clear();
    prepared_nodes.nodes.clear();

    for extracted_node in &extracted_nodes.items {
        if let UiBackground::SolidColor { color } = extracted_node.material {
            let position = extracted_node.layout.position;
            let size = extracted_node.layout.size;
            let model_matrix =
                Mat4::from_translation(position.extend(0.0)) * Mat4::from_scale(size.extend(1.0));

            let dynamic_offset = instance_buffer.instances.len() as u32 * instance_buffer.stride;

            instance_buffer.instances.push(UiInstanceData {
                model_matrix: model_matrix.to_cols_array(),
                color: color,
            });

            prepared_nodes.nodes.push(PreparedUiNode { dynamic_offset });
        }
    }

    // TODO: check if buffer needs to be resized.
    for (i, instance) in instance_buffer.instances.iter().enumerate() {
        let offset = (i as u64) * (instance_buffer.stride as u64);
        let bytes = bytemuck::bytes_of(instance);
        gfx.context
            .queue
            .write_buffer(&instance_buffer.buffer, offset, bytes);
    }
}
