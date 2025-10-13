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

// INFO: -----------------
//         Systems
// -----------------------

pub fn queue_ui_system(
    // Input
    gfx: Res<GraphicsContextResource>,
    mut extracted_panels: ResMut<ExtractedBy<UiPanelExtractor>>,
    mut extracted_texts: ResMut<ExtractedBy<UiTextExtractor>>,

    // In/Out (persistent storage buffer)
    mut sort_buffer: ResMut<UiElementSortBufferResource>,

    // Output (buffers)
    mut material_buffer: ResMut<UiMaterialBuffer>,
    mut object_buffer: ResMut<UiObjectBuffer>,
    mut prepared_batches: ResMut<PreparedUiBatches>,
) {
    material_buffer.materials.clear();
    object_buffer.objects.clear();
    prepared_batches.batches.clear();
    sort_buffer.clear();

    // INFO: ---------------------------------
    //         Sort all items by depth
    // ---------------------------------------

    debug!(
        target : "ui_efficiency",
        "Preparing UI batches (only needs to run if UI updated)..."
    );

    // We use a sort buffer and extend it with all extracted items
    // since this is quicker than re-allocating a new vec every frame.
    // This flushes the extracted items, but since this is the only
    // system that requires them it shouldn't be a problem I'm hoping.
    sort_buffer.extend(extracted_panels.items.drain(..));
    sort_buffer.extend(extracted_texts.items.drain(..));

    // Sort unstably for faster sorting. If two UI elements overlap on the same
    // depth this may lead to flickering but for now I'm just considering that
    // developer error, you should adjust the Z index for those or something.
    sort_buffer.sort_unstable_by(|a, b| a.sort_key.total_cmp(&b.sort_key));

    if sort_buffer.is_empty() {
        return;
    }

    debug!(target: "ui_efficiency", "Sorted {} UI elements for batching.", sort_buffer.len());

    // INFO: -----------------------------------
    //         Create UI Batches for GPU
    // -----------------------------------------

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

                // check if this panel can be part of the current panel batch
                if current_panel_material_color == Some(color_key) && current_panel_batch.is_some()
                {
                    // it can, just extend the instance count
                    let batch = current_panel_batch.as_mut().unwrap();
                    batch.instance_count += 1;
                } else {
                    // it can't, so flush the old panel batch and start a new one
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

                // ensure a text batch is active
                if current_text_batch.is_none() {
                    current_text_batch = Some(TextBatch::default());
                }

                debug!(target: "ui_batching", "Added text to batch: {:?}", item.kind);

                // add the text to the current text batch
                current_text_batch
                    .as_mut()
                    .unwrap()
                    .texts
                    .push(item.kind.clone());
            }
        }
    }

    // flush any remaining batches after the loop
    flush_panel_batch(current_panel_batch.take(), &mut prepared_batches.batches);
    flush_text_batch(current_text_batch.take(), &mut prepared_batches.batches);

    debug!("Prepared {} UI batches", prepared_batches.batches.len());

    // write material data to GPU buffers
    for (i, material) in material_buffer.materials.iter().enumerate() {
        let offset = (i as u64) * (material_buffer.stride as u64);
        let bytes = bytemuck::bytes_of(material);
        gfx.context
            .queue
            .write_buffer(&material_buffer.buffer, offset, bytes);
    }

    // write object data to GPU buffer
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
