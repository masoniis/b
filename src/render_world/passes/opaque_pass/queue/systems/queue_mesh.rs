use crate::{
    prelude::*,
    render_world::{
        global_extract::{
            components::mesh::{RenderMeshComponent, RenderTransformComponent},
            resources::RenderCameraResource,
        },
        passes::opaque_pass::queue::{Opaque3dRenderPhase, PhaseItem},
    },
};
use bevy_ecs::prelude::*;

/// The system responsible for populating the `RenderQueueResource`.
///
/// Performs a query for all entities that have been extracted into the render
/// world and adds them to a list of draw calls for the renderer to consume.
#[instrument(skip_all)]
pub fn queue_mesh_system(
    // Input
    camera_info: Res<RenderCameraResource>,
    meshes_query: Query<(Entity, &RenderMeshComponent, &RenderTransformComponent)>,

    // Output
    mut opaque_phase: ResMut<Opaque3dRenderPhase>,
) {
    opaque_phase.items.clear();

    let camera_position = camera_info.world_position;

    for (entity, _mesh, transform) in meshes_query.iter() {
        // TODO: Implement frustum culling. Check if the mesh's bounding box
        // is visible from the camera's perspective.
        // let is_visible = frustum_cull(transform, &camera_info.frustum);
        // if !is_visible {
        //     continue;
        // }

        let object_position = transform.transform.w_axis.truncate();
        let distance_from_camera = (object_position - camera_position).length_squared();

        opaque_phase.items.push(PhaseItem {
            entity,
            distance: distance_from_camera,
        });
    }

    // --- 3. Sorting ---
    // For opaque objects, sort front-to-back (descending distance) to maximize early-Z culling.
    opaque_phase
        .items
        .sort_by(|a, b| b.distance.partial_cmp(&a.distance).unwrap());
}
