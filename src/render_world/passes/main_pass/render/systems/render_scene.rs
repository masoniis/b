use crate::{
    prelude::*,
    render_world::{
        extract::{
            extract_meshes::{RenderMeshComponent, RenderTransformComponent},
            RenderMeshStorageResource,
        },
        passes::main_pass::{
            prepare::{
                bind_groups::ModelBindGroup,
                resources::{DepthTextureResource, MainTextureBindGroup, ViewBindGroup},
                systems::prepare_pipelines::MESH_PIPELINE_ID,
            },
            queue::Opaque3dRenderPhase,
        },
        resources::{GraphicsContextResource, PipelineCacheResource},
        uniforms::ModelUniform,
    },
};
use bevy_ecs::prelude::*;
use bevy_ecs::system::Res;
use wgpu::TextureViewDescriptor;

/// The main rendering system, now acting as a direct "executor" of a render phase.
pub fn render_scene_system(
    // --- INPUTS ---
    // Core GPU context
    gfx_resource: Res<GraphicsContextResource>,
    // The "playlist" of what to draw for this frame
    phase: Res<Opaque3dRenderPhase>,
    // All the GPU resources needed to draw the items in the playlist
    pipeline_cache: Res<PipelineCacheResource>,
    mesh_storage: Res<RenderMeshStorageResource>,
    view_bind_group: Res<ViewBindGroup>,
    texture_bind_group: Res<MainTextureBindGroup>,
    depth_texture: Res<DepthTextureResource>,
    mesh_query: Query<&RenderMeshComponent>,
    model_bind_group: Res<ModelBindGroup>,
    transforms: Query<&RenderTransformComponent>,
) {
    let gfx = &gfx_resource.context;

    // 1. Get the surface texture to draw into. This part is the same.
    let output = match gfx.surface.get_current_texture() {
        Ok(texture) => texture,
        Err(e) => {
            error!("Error acquiring surface texture: {:?}", e);
            return;
        }
    };
    let view = output
        .texture
        .create_view(&TextureViewDescriptor::default());

    // 2. Create a command encoder.
    let mut encoder = gfx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Scene Encoder"),
        });

    let mut model_uniform_data: Vec<ModelUniform> = Vec::with_capacity(phase.items.len());
    for item in &phase.items {
        if let Ok(transform_comp) = transforms.get(item.entity) {
            model_uniform_data.push(ModelUniform::new(
                transform_comp.transform.to_cols_array_2d(),
            ));
        }
    }

    // Upload all model matrices to the GPU in ONE operation
    if !model_uniform_data.is_empty() {
        gfx.queue.write_buffer(
            &model_bind_group.buffer,
            0,
            bytemuck::cast_slice(&model_uniform_data),
        );
    }

    // 3. Get the compiled pipeline from the cache.
    let pipeline = pipeline_cache.get(MESH_PIPELINE_ID).unwrap();

    // 4. Begin the render pass. This is where the drawing commands are recorded.
    {
        // Scoped to drop the mutable borrow of `encoder`
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Main Scene Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0), // Clear depth to max distance
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // --- THE CORE RENDER LOOP ---

        // 5. Set the state that is common for the entire pass.
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &view_bind_group.bind_group, &[]); // Camera data
        render_pass.set_bind_group(1, &texture_bind_group.0, &[]); // Texture atlas data

        for (i, item) in phase.items.iter().enumerate() {
            if let Ok(render_mesh_comp) = mesh_query.get(item.entity) {
                if let Some(gpu_mesh) = mesh_storage.meshes.get(&render_mesh_comp.mesh_handle.id())
                {
                    // 1. Calculate the offset using the CORRECT stride from the resource
                    let dynamic_offset = (i * std::mem::size_of::<ModelUniform>()) as u32;
                    let aligned_offset = (dynamic_offset + 255) & !255;

                    // 3. Set the bind group WITH the dynamic offset
                    render_pass.set_bind_group(2, &model_bind_group.bind_group, &[aligned_offset]);

                    // 4. Set buffers and draw
                    render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(
                        gpu_mesh.index_buffer.slice(..),
                        wgpu::IndexFormat::Uint32,
                    );
                    render_pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
                }
            }
        }
    }

    // 7. Submit the command buffer to the GPU and present the frame.
    gfx.queue.submit(std::iter::once(encoder.finish()));
    output.present();
}
