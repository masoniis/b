use crate::{
    game_world::ui::components::UiBackground,
    prelude::*,
    render_world::{
        extract::{extract_component::ExtractedItems, ui::UiNodeExtractor},
        passes::ui_pass::startup::{
            UiMaterialBuffer, UiMaterialData, UiObjectBuffer, UiObjectData,
        },
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct PreparedUiBatches {
    pub batches: Vec<UiBatch>,
}

#[derive(Clone, Copy, Default)]
pub struct UiBatch {
    pub material_index: u32,
    pub first_instance: u32,
    pub instance_count: u32,
}

pub fn prepare_ui_batches_system(
    gfx: Res<GraphicsContextResource>,
    extracted_nodes: Res<ExtractedItems<UiNodeExtractor>>,
    mut material_buffer: ResMut<UiMaterialBuffer>,
    mut object_buffer: ResMut<UiObjectBuffer>,
    mut prepared_batches: ResMut<PreparedUiBatches>,
) {
    material_buffer.materials.clear();
    object_buffer.objects.clear();
    prepared_batches.batches.clear();

    let mut intermediate_nodes: Vec<_> = extracted_nodes
        .items
        .iter()
        .filter_map(|extracted_node| {
            if let UiBackground::SolidColor { color } = extracted_node.material {
                let position = extracted_node.layout.position;
                let size = extracted_node.layout.size;
                let model_matrix = Mat4::from_translation(position.extend(0.0))
                    * Mat4::from_scale(size.extend(1.0));

                Some((
                    (color.map(|f| f.to_bits())),
                    UiObjectData {
                        model_matrix: model_matrix.to_cols_array(),
                    },
                ))
            } else {
                None
            }
        })
        .collect();

    // Sort by material color
    intermediate_nodes.sort_by_key(|(color_key, _)| *color_key);

    let mut current_material_key = None;
    let mut current_batch: Option<UiBatch> = None;

    for (color_key, object_data) in intermediate_nodes {
        let object_index = object_buffer.objects.len() as u32;
        object_buffer.objects.push(object_data);

        if Some(color_key) != current_material_key {
            if let Some(batch) = current_batch.take() {
                prepared_batches.batches.push(batch);
            }

            let material_index = material_buffer.materials.len() as u32;
            let color = [
                f32::from_bits(color_key[0]),
                f32::from_bits(color_key[1]),
                f32::from_bits(color_key[2]),
                f32::from_bits(color_key[3]),
            ];
            material_buffer.materials.push(UiMaterialData { color });

            current_batch = Some(UiBatch {
                material_index,
                first_instance: object_index,
                instance_count: 1,
            });
            current_material_key = Some(color_key);
        } else if let Some(batch) = &mut current_batch {
            batch.instance_count += 1;
        }
    }
    if let Some(batch) = current_batch.take() {
        prepared_batches.batches.push(batch);
    }

    // Write data to GPU
    for (i, material) in material_buffer.materials.iter().enumerate() {
        let offset = (i as u64) * (material_buffer.stride as u64);
        let bytes = bytemuck::bytes_of(material);
        gfx.context
            .queue
            .write_buffer(&material_buffer.buffer, offset, bytes);
    }

    if !object_buffer.objects.is_empty() {
        let object_bytes = bytemuck::cast_slice(&object_buffer.objects);
        gfx.context
            .queue
            .write_buffer(&object_buffer.buffer, 0, object_bytes);
    }
}
