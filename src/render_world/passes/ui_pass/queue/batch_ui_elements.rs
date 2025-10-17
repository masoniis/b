use std::collections::HashMap;

use crate::{
    prelude::*,
    render_world::{
        passes::ui_pass::{
            extract::{RenderableUiElement, UiElementKind},
            prepare::UiChanges,
            startup::{UiMaterialBuffer, UiMaterialData, UiObjectBuffer, UiObjectData},
        },
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;
use derive_more::{Deref, DerefMut};

// INFO: -------------------
//         Resources
// -------------------------

/// A batch of panels sharing the same material
#[derive(Clone, Copy, Debug)]
pub struct PanelBatch {
    pub material_index: u32,
    pub first_instance: u32,
    pub instance_count: u32,
}

/// A batch of text elements to be rendered together by glyphon
#[derive(Clone, Debug, Default)]
pub struct TextBatch {
    pub texts: Vec<UiElementKind>,
}

/// An enum representing a batch of either panels or text
#[derive(Clone, Debug)]
pub enum UiRenderBatch {
    Panel(PanelBatch),
    Text(TextBatch),
}

/// A vector of prepared UI render batches ready for rendering.
///
/// They should be ordered by depth (back to front).
#[derive(Resource, Default, Deref, DerefMut)]
pub struct PreparedUiBatches {
    pub batches: Vec<UiRenderBatch>,
}

/// A buffer used to store UI elements for sorting.
///
/// By using a buffer, it is guaranteed no new vec is
/// allocated every time which improves performance.
#[derive(Resource, Default, Deref, DerefMut)]
pub struct UiElementSortBufferResource(Vec<RenderableUiElement>);

/// A persistent cache of all renderable UI elements in the render world.
/// This is the "single source of truth" for the UI batching system.
#[derive(Resource, Default)]
pub struct UiElementCache {
    pub elements: HashMap<Entity, RenderableUiElement>,
}

/// A marker resource that indicates whether the glyphon text atlas and buffers need to be updated.
#[derive(Resource, Default, Deref, DerefMut, PartialEq)]
pub struct IsGlyphonDirty(pub bool);

// INFO: -----------------
//         Systems
// -----------------------

// The system now needs the UiChanges resource to make decisions
#[instrument(skip_all)]
pub fn rebuild_ui_batches_system(
    // Inputs
    gfx: Res<GraphicsContextResource>,
    element_cache: Res<UiElementCache>,
    mut sort_buffer: ResMut<UiElementSortBufferResource>,
    ui_changes: Res<UiChanges>, // The decider

    // Outputs
    mut material_buffer: ResMut<UiMaterialBuffer>,
    mut object_buffer: ResMut<UiObjectBuffer>,
    mut prepared_batches: ResMut<PreparedUiBatches>,
) {
    if ui_changes.structural_change_occured || ui_changes.panel_content_change_occured {
        debug!(
            target: "ui_efficiency",
            "Panel data changed. Re-uploading panel GPU buffers..."
        );

        material_buffer.materials.clear();
        object_buffer.objects.clear();

        // sort all elements to get a stable order for processing panels
        sort_buffer.clear();
        sort_buffer.extend(element_cache.elements.values().cloned());
        sort_buffer.sort_unstable_by(|a, b| a.sort_key.total_cmp(&b.sort_key));

        let mut material_map: HashMap<[u32; 4], u32> = HashMap::new();
        for element in sort_buffer.iter() {
            if let UiElementKind::Panel {
                color,
                position,
                size,
            } = &element.kind
            {
                // add new materials if they haven't been seen yet
                let color_key = color.map(|f| f.to_bits());
                material_map.entry(color_key).or_insert_with(|| {
                    let new_material_index = material_buffer.materials.len() as u32;
                    material_buffer
                        .materials
                        .push(UiMaterialData { color: *color });
                    new_material_index
                });

                // add the panel's transform to the object buffer
                let model_matrix = Mat4::from_translation(position.extend(0.0))
                    * Mat4::from_scale(size.extend(1.0));
                object_buffer.objects.push(UiObjectData {
                    model_matrix: model_matrix.to_cols_array(),
                });
            }
        }

        // write to the GPU buffers
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

    prepared_batches.batches.clear();
    sort_buffer.clear();
    sort_buffer.extend(element_cache.elements.values().cloned());
    sort_buffer.sort_unstable_by(|a, b| a.sort_key.total_cmp(&b.sort_key));

    if sort_buffer.is_empty() {
        return;
    }

    // rebuild the material map from the buffer's current state (it's either new or from a previous frame
    let material_map: HashMap<[u32; 4], u32> = material_buffer
        .materials
        .iter()
        .enumerate()
        .map(|(i, mat)| (mat.color.map(|f| f.to_bits()), i as u32))
        .collect();

    let mut current_panel_batch: Option<PanelBatch> = None;
    let mut current_panel_material_color: Option<[u32; 4]> = None;
    let mut current_text_batch: Option<TextBatch> = None;
    let mut object_index_counter = 0;

    for item in sort_buffer.drain(..) {
        match &item.kind {
            UiElementKind::Panel { color, .. } => {
                flush_text_batch(current_text_batch.take(), &mut prepared_batches.batches);

                let color_key = color.map(|f| f.to_bits());
                if current_panel_material_color == Some(color_key) && current_panel_batch.is_some()
                {
                    current_panel_batch.as_mut().unwrap().instance_count += 1;
                } else {
                    flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);

                    let material_index = *material_map
                        .get(&color_key)
                        .expect("Material should exist in map");
                    current_panel_batch = Some(PanelBatch {
                        material_index,
                        first_instance: object_index_counter,
                        instance_count: 1,
                    });
                    current_panel_material_color = Some(color_key);
                }
                object_index_counter += 1;
            }
            UiElementKind::Text { .. } => {
                flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);
                current_panel_material_color = None;

                if current_text_batch.is_none() {
                    current_text_batch = Some(TextBatch::default());
                }
                current_text_batch
                    .as_mut()
                    .unwrap()
                    .texts
                    .push(item.kind.clone());
            }
        }
    }

    flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);
    flush_text_batch(current_text_batch.take(), &mut prepared_batches.batches);
}

/// Flushes a panel batch into the list of render batches if it exists.
fn flush_panel_batch(batch: Option<PanelBatch>, batches: &mut Vec<UiRenderBatch>) {
    if let Some(batch) = batch {
        batches.push(UiRenderBatch::Panel(batch));
    }
}

/// Flushes a panel batch into the list of render batches if it exists.
fn flush_text_batch(batch: Option<TextBatch>, batches: &mut Vec<UiRenderBatch>) {
    if let Some(batch) = batch {
        if !batch.texts.is_empty() {
            batches.push(UiRenderBatch::Text(batch));
        }
    }
}
