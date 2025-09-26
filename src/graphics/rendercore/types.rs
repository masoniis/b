use crate::{
    ecs::resources::{
        asset_storage::{AssetId, Handle, MeshAsset},
        AssetStorageResource, CameraUniformResource, RenderQueueResource,
    },
    graphics::{
        renderpass::{RenderContext, SharedRenderData},
        GpuMesh,
    },
};
use std::{collections::HashMap, sync::Arc};
use wgpu::{Device, Queue, RenderPipeline};

pub const SHADER_PATH: &str = "src/assets/shaders/scene/simple.wgsl";
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const MAX_TRANSFORMS: u64 = 100000;

pub struct WebGpuRenderer {
    // Core
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub render_pipeline: RenderPipeline,

    // Render Passes
    pub passes: Vec<crate::graphics::renderpass::RenderPass>,

    // Shared Data
    pub shared_data: SharedRenderData,

    // Buffers
    pub depth_texture_view: wgpu::TextureView,
    pub instance_buffer: wgpu::Buffer,

    // Assets
    pub gpu_meshes: HashMap<AssetId, Arc<GpuMesh>>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model_matrix: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2, // model_row_0
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3, // model_row_1
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 4]>() * 2) as wgpu::BufferAddress,
                    shader_location: 4, // model_row_2
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 4]>() * 3) as wgpu::BufferAddress,
                    shader_location: 5, // model_row_3
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

pub struct QueuedDraw {
    pub mesh_handle: Handle<MeshAsset>,
    pub instance_count: u32,
    pub transform: glam::Mat4,
}

pub trait ISceneRenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    );

    fn render<'a>(
        &'a self,
        encoder: &mut wgpu::CommandEncoder,
        context: RenderContext<'a>,
        ecs_render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        instance_buffer: &wgpu::Buffer,
        gpu_meshes: &mut HashMap<AssetId, Arc<GpuMesh>>,
        render_pipeline: &wgpu::RenderPipeline,
    );
}

pub trait ITextRenderPass {
    fn prepare(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        render_queue: &RenderQueueResource,
        mesh_assets: &AssetStorageResource<MeshAsset>,
        camera_uniform: &CameraUniformResource,
    );

    fn render<'a>(&'a self, encoder: &mut wgpu::CommandEncoder, context: RenderContext<'a>);
}
