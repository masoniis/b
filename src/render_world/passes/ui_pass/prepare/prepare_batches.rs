use crate::{
    prelude::*,
    render_world::{
        extract::{
            extract_component::ExtractedBy,
            ui::{RenderableUiElement, UiElementKind, UiPanelExtractor, UiTextExtractor},
        },
        passes::ui_pass::startup::{
            UiMaterialBuffer, UiMaterialData, UiObjectBuffer, UiObjectData,
        },
        resources::GraphicsContextResource,
    },
};
use bevy_ecs::prelude::*;
use std::{
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

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
#[derive(Resource, Default)]
pub struct PreparedUiBatches {
    pub batches: Vec<UiRenderBatch>,
}

impl Deref for TextBatch {
    type Target = Vec<UiElementKind>;
    fn deref(&self) -> &Self::Target {
        &self.texts
    }
}

impl DerefMut for TextBatch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.texts
    }
}

// INFO: -----------------
//         Systems
// -----------------------

pub fn prepare_ui_batches_system(
    // Input
    gfx: Res<GraphicsContextResource>,
    extracted_panels: Res<ExtractedBy<UiPanelExtractor>>,
    extracted_texts: Res<ExtractedBy<UiTextExtractor>>,

    // Output (buffers)
    mut material_buffer: ResMut<UiMaterialBuffer>,
    mut object_buffer: ResMut<UiObjectBuffer>,
    mut prepared_batches: ResMut<PreparedUiBatches>,
) {
    material_buffer.materials.clear();
    object_buffer.objects.clear();
    prepared_batches.batches.clear();

    // INFO: ---------------------------------
    //         Sort all items by depth
    // ---------------------------------------

    debug!(
        target : "ui_efficiency",
        "Preparing UI batches (this should only happen if UI updated)..."
    );

    let mut all_items: Vec<&RenderableUiElement> = extracted_panels
        .items
        .iter()
        .chain(extracted_texts.items.iter())
        .collect();
    all_items.sort_by(|a, b| {
        a.sort_key
            .partial_cmp(&b.sort_key)
            .unwrap_or(Ordering::Equal)
    });

    if all_items.is_empty() {
        return;
    }

    // INFO: -----------------------------------
    //         Create UI Batches for GPU
    // -----------------------------------------

    let mut current_panel_batch: Option<PanelBatch> = None;
    let mut current_panel_material_color: Option<[u32; 4]> = None;
    let mut current_text_batch: Option<TextBatch> = None;

    for item in all_items {
        match &item.kind {
            UiElementKind::Panel {
                color,
                position,
                size,
            } => {
                flush_text_batch(current_text_batch.take(), &mut prepared_batches.batches);

                let color_key = color.map(|f| f.to_bits());
                let object_index = object_buffer.objects.len() as u32;

                // Check if this panel can be part of the current panel batch
                if current_panel_material_color == Some(color_key) && current_panel_batch.is_some()
                {
                    // It can, just extend the instance count
                    let batch = current_panel_batch.as_mut().unwrap();
                    batch.instance_count += 1;
                } else {
                    // It can't, so flush the old panel batch and start a new one
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

                // Add the panel's data to the object buffer
                let model_matrix = Mat4::from_translation(position.extend(0.0))
                    * Mat4::from_scale(size.extend(1.0));
                object_buffer.objects.push(UiObjectData {
                    model_matrix: model_matrix.to_cols_array(),
                });
            }
            UiElementKind::Text { .. } => {
                flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);
                current_panel_material_color = None;

                // Ensure a text batch is active
                if current_text_batch.is_none() {
                    current_text_batch = Some(TextBatch::default());
                }

                // Add the text to the current text batch
                current_text_batch
                    .as_mut()
                    .unwrap()
                    .texts
                    .push(item.kind.clone());
            }
        }
    }

    // 3. Flush any remaining batches after the loop
    flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);
    flush_text_batch(current_text_batch.take(), &mut prepared_batches.batches);

    debug!("Prepared {} UI batches", prepared_batches.batches.len());

    // 4. Write panel data to GPU buffers
    // The text data in `PreparedUiBatches` will be used by a separate text render pass
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
    if let Some(b) = batch {
        batches.push(UiRenderBatch::Panel(b));
    }
}

/// Flushes a panel batch into the list of render batches if it exists.
fn flush_text_batch(batch: Option<TextBatch>, batches: &mut Vec<UiRenderBatch>) {
    if let Some(b) = batch {
        if !b.texts.is_empty() {
            batches.push(UiRenderBatch::Text(b));
        }
    }
}
