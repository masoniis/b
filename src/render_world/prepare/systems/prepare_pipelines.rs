use crate::render_world::{
    prepare::resources::{pipeline_cache::PipelineId, PipelineCacheResource},
    render::GraphicsContextResource,
};
use bevy_ecs::prelude::*;

pub const MESH_PIPELINE_ID: PipelineId = 0;
pub const LOADING_SCREEN_PIPELINE_ID: PipelineId = 1;

// This system runs once or whenever it needs to
pub fn prepare_pipelines_system(
    mut cache: ResMut<PipelineCacheResource>,
    device: Res<GraphicsContextResource>, // Assuming you have this
) {
    // Only create the pipeline if it doesn't exist
    if cache.get(MESH_PIPELINE_ID).is_none() {
        let device = &device.context.device;
        // let mesh_pipeline = device.create_render_pipeline(...);
        // cache.insert(MESH_PIPELINE_ID, mesh_pipeline);
    }

    if cache.get(LOADING_SCREEN_PIPELINE_ID).is_none() {}
}
