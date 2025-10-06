use crate::{render_world::context::GraphicsContext, render_world::textures::TextureArray};
use bevy_ecs::prelude::Resource;
use std::collections::HashMap;
use wgpu::RenderPipeline;

#[derive(Resource)]
pub struct GraphicsContextResource {
    pub context: GraphicsContext,
}

#[derive(Resource)]
pub struct TextureArrayResource {
    pub array: TextureArray,
}

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
