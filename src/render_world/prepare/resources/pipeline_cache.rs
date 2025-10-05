use bevy_ecs::prelude::Resource;
use std::collections::HashMap;
use wgpu::RenderPipeline;

pub type PipelineId = u64;

#[derive(Resource, Default)]
pub struct PipelineCacheResource {
    pipelines: HashMap<PipelineId, RenderPipeline>,
}

impl PipelineCacheResource {
    pub fn get(&self, id: PipelineId) -> Option<&RenderPipeline> {
        self.pipelines.get(&id)
    }

    pub fn insert(&mut self, id: PipelineId, pipeline: RenderPipeline) {
        self.pipelines.insert(id, pipeline);
    }
}
