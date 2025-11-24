use crate::{
    prelude::*,
    render_world::{
        global_extract::resources::RenderCameraResource,
        graphics_context::resources::{RenderDevice, RenderQueue},
        passes::world::main_passes::{
            opaque_pass::extract::RenderTransformComponent,
            transparent_pass::{
                extract::TransparentRenderMeshComponent, startup::TransparentPipeline,
            },
        },
    },
};
use bevy_ecs::prelude::*;

#[derive(Debug)]
pub struct PhaseItem {
    pub entity: Entity,
    pub distance: f32, // for sorting back-to-front
}

#[derive(Resource, Default)]
pub struct Transparent3dRenderPhase {
    pub items: Vec<PhaseItem>,
}

/// The system responsible for populating the `RenderQueueResource`.
///
/// Performs a query for all entities that have been extracted into the render
/// world and adds them to a list of draw calls for the renderer to consume.
#[instrument(skip_all)]
pub fn queue_and_prepare_transparent_system(
    // Input
    _device: Res<RenderDevice>,
    _queue: Res<RenderQueue>,
    _pipeline: Res<TransparentPipeline>,
    camera_info: Res<RenderCameraResource>,
    meshes_query: Query<(
        Entity,
        &TransparentRenderMeshComponent,
        &RenderTransformComponent,
    )>,

    // Output
    mut transparent_phase: ResMut<Transparent3dRenderPhase>,
) {
    transparent_phase.items.clear();

    // collect sortable items for the render pass
    let camera_position = camera_info.world_position;
    let mut sortable_items: Vec<PhaseItem> = Vec::with_capacity(meshes_query.iter().len());
    for (entity, _mesh, transform) in meshes_query.iter() {
        // TODO: Frustum culling here

        let object_position = transform.transform.w_axis.truncate();
        let distance_from_camera = (object_position - camera_position).length_squared();

        sortable_items.push(PhaseItem {
            distance: distance_from_camera,
            entity,
        });
    }

    // sort by back to front for transparency
    sortable_items.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

    // populate the phase and object buffer in correct sorted order
    for item in sortable_items {
        transparent_phase.items.push(PhaseItem {
            entity: item.entity,
            distance: item.distance,
        });
    }
}
