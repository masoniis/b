use crate::prelude::*;
use crate::simulation_world::{
    asset_management::{AssetStorageResource, MeshAsset},
    chunk::MeshComponent,
    user_interface::components::UiText,
    user_interface::screens::debug_screen::{
        IndexCountTextMarker, MeshCountTextMarker, VertexCountTextMarker,
    },
};
use bevy_ecs::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct MeshCounterResource {
    pub total_meshes: usize,
    pub total_vertices: usize,
    pub total_indices: usize,
}

/// Observes the removal of MeshComponent and updates the MeshCounterResource accordingly.
#[instrument(skip_all)]
pub fn mesh_remove_observer(
    trigger: On<Remove, MeshComponent>,

    // Input
    asset_storage: Res<AssetStorageResource<MeshAsset>>,
    mesh_query: Query<&MeshComponent>,

    // Output (updated counts)
    mut mesh_count: ResMut<MeshCounterResource>,
) {
    let entity = trigger.entity;

    if let Ok(mesh_component) = mesh_query.get(entity) {
        if let Some(mesh) = asset_storage.get(mesh_component.mesh_handle) {
            // use saturating_sub to prevent panicking
            mesh_count.total_meshes = mesh_count.total_meshes.saturating_sub(1);
            mesh_count.total_vertices = mesh_count
                .total_vertices
                .saturating_sub(mesh.vertices.len());
            mesh_count.total_indices = mesh_count.total_indices.saturating_sub(mesh.indices.len());
        } else {
            warn!(
                "MeshComponentRemovedMessage received for an invalid handle: {:?}",
                mesh_component.mesh_handle.id()
            );
        }
    }
}

/// Observes the addition of MeshComponent and updates the MeshCounterResource accordingly.
#[instrument(skip_all)]
pub fn mesh_add_observer(
    trigger: On<Add, MeshComponent>,

    // Input
    asset_storage: Res<AssetStorageResource<MeshAsset>>,
    mesh_query: Query<&MeshComponent>,

    // Output
    mut mesh_count: ResMut<MeshCounterResource>,
) {
    let entity = trigger.entity;

    if let Ok(mesh_component) = mesh_query.get(entity) {
        if let Some(mesh) = asset_storage.get(mesh_component.mesh_handle) {
            mesh_count.total_meshes += 1;
            mesh_count.total_vertices += mesh.vertices.len();
            mesh_count.total_indices += mesh.indices.len();
        } else {
            warn!(
                "MeshComponent added with an invalid handle: {:?}",
                mesh_component.mesh_handle.id()
            );
        }
    }
}

/// Updates the content of the Mesh counter text element when the resource changes.
#[instrument(skip_all)]
pub fn update_mesh_counter_screen_text_system(
    // Input
    mesh_counter: Res<MeshCounterResource>,

    // Output (updated UI)
    mut text_query: Query<(
        &mut UiText,
        Option<&MeshCountTextMarker>,
        Option<&VertexCountTextMarker>,
        Option<&IndexCountTextMarker>,
    )>,
) {
    for (mut text, mesh_marker, vertex_marker, index_marker) in text_query.iter_mut() {
        if mesh_marker.is_some() {
            text.content = format!("{}", mesh_counter.total_meshes);
        } else if vertex_marker.is_some() {
            text.content = format!("{}", mesh_counter.total_vertices);
        } else if index_marker.is_some() {
            text.content = format!("{}", mesh_counter.total_indices);
        }
    }
}
