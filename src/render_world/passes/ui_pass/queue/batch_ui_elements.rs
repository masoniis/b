use std::collections::HashMap;

use crate::{
    prelude::*,
    render_world::{
        passes::ui_pass::{
            extract::{RenderableUiElement, UiElementKind},
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

pub fn rebuild_ui_batches_system(
    // Inputs
    gfx: Res<GraphicsContextResource>,
    element_cache: Res<UiElementCache>,
    mut sort_buffer: ResMut<UiElementSortBufferResource>,

    // Outputs
    mut material_buffer: ResMut<UiMaterialBuffer>,
    mut object_buffer: ResMut<UiObjectBuffer>,
    mut prepared_batches: ResMut<PreparedUiBatches>,
) {
    debug!(
        target: "ui_efficiency",
        "Structural or panel content change detected. Performing full batch rebuild..."
    );

    material_buffer.materials.clear();
    object_buffer.objects.clear();
    prepared_batches.batches.clear();
    sort_buffer.clear();

    sort_buffer.extend(element_cache.elements.values().cloned());
    sort_buffer.sort_unstable_by(|a, b| a.sort_key.total_cmp(&b.sort_key));

    if sort_buffer.is_empty() {
        return;
    }

    debug!(
        target: "ui_efficiency",
        "Sorted buffer wasn't empty, {} UI elements have been sorted for batching.",
        sort_buffer.len()
    );

    let mut current_panel_batch: Option<PanelBatch> = None;
    let mut current_panel_material_color: Option<[u32; 4]> = None;
    let mut current_text_batch: Option<TextBatch> = None;

    for item in sort_buffer.drain(..) {
        match &item.kind {
            UiElementKind::Panel {
                color,
                position,
                size,
            } => {
                flush_text_batch(current_text_batch.take(), &mut prepared_batches.batches);

                let color_key = color.map(|f| f.to_bits());
                let object_index = object_buffer.objects.len() as u32;

                if current_panel_material_color == Some(color_key) && current_panel_batch.is_some()
                {
                    let batch = current_panel_batch.as_mut().unwrap();
                    batch.instance_count += 1;
                } else {
                    flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);

                    let material_index = material_buffer.materials.len() as u32;
                    material_buffer
                        .materials
                        .push(UiMaterialData { color: *color });

                    current_panel_batch = Some(PanelBatch {
                        material_index,
                        first_instance: object_index,
                        instance_count: 1,
                    });
                    current_panel_material_color = Some(color_key);
                }

                let model_matrix = Mat4::from_translation(position.extend(0.0))
                    * Mat4::from_scale(size.extend(1.0));
                object_buffer.objects.push(UiObjectData {
                    model_matrix: model_matrix.to_cols_array(),
                });
            }
            UiElementKind::Text { .. } => {
                flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);
                current_panel_material_color = None;

                if current_text_batch.is_none() {
                    current_text_batch = Some(TextBatch::default());
                }

                debug!(target: "ui_batching", "Added text to batch: {:?}", item.kind);

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

    debug!("Prepared {} UI batches", prepared_batches.batches.len());

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
