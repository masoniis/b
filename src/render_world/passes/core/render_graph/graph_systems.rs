use crate::prelude::*;
use crate::render_world::passes::world::shadow_pass::render::ShadowRenderPassNode;
use crate::render_world::{
    graphics_context::resources::{RenderDevice, RenderQueue, RenderSurface, RenderSurfaceConfig},
    passes::{
        core::{RenderContext, RenderGraph},
        ui_pass::UiRenderPassNode,
        world::main_passes::{
            bounding_box_pass::BoundingBoxNode, opaque_pass::OpaquePassRenderNode,
            transparent_pass::TransparentPassRenderNode,
        },
    },
};
use bevy_ecs::prelude::*;

// INFO: --------------------------------------------------------
//         Systems to set up and execute the render graph
// --------------------------------------------------------------

/// An exclusive system that runs once at startup to create, configure,
/// and insert the application's RenderGraph resource.
pub fn setup_render_graph(world: &mut World) {
    let mut render_graph = RenderGraph::default();

    let shadow_pass_node = ShadowRenderPassNode::new(world);
    let opaque_pass_node = OpaquePassRenderNode::new(world);
    let transparent_pass_node = TransparentPassRenderNode::new(world);
    let bounding_box_node = BoundingBoxNode;
    let ui_pass_node = UiRenderPassNode;

    render_graph.add_node::<ShadowRenderPassNode, _>("ShadowPass", shadow_pass_node, true);
    render_graph.add_node::<OpaquePassRenderNode, _>("OpaquePass", opaque_pass_node, true);
    #[rustfmt::skip]
    render_graph.add_node::<TransparentPassRenderNode, _>("TransparentPass", transparent_pass_node, true);
    render_graph.add_node::<UiRenderPassNode, _>("UiPass", ui_pass_node, true);
    render_graph.add_node::<BoundingBoxNode, _>("WireframePass", bounding_box_node, true);

    render_graph.add_node_dependency::<OpaquePassRenderNode, ShadowRenderPassNode>();
    render_graph.add_node_dependency::<TransparentPassRenderNode, OpaquePassRenderNode>();
    render_graph.add_node_dependency::<BoundingBoxNode, TransparentPassRenderNode>();
    render_graph.add_node_dependency::<UiRenderPassNode, TransparentPassRenderNode>();

    world.insert_resource(render_graph);

    info!("Render graph created and configured!");
}

#[instrument(skip_all)]
pub fn execute_render_graph_system(world: &mut World) {
    // take ownership of the graph
    let Some(mut render_graph) = world.remove_resource::<RenderGraph>() else {
        return;
    };

    let (Some(device), Some(queue), Some(surface), Some(config)) = (
        world.get_resource::<RenderDevice>(),
        world.get_resource::<RenderQueue>(),
        world.get_resource::<RenderSurface>(),
        world.get_resource::<RenderSurfaceConfig>(),
    ) else {
        world.insert_resource(render_graph);
        warn!("Couldn't get one or more required render resources (Device, Queue, Surface, or Config) to execute the render graph!");
        return;
    };

    // clone the Arcs to satisfy lifetimes
    let device = device.0.clone();
    let queue = queue.0.clone();
    let surface = surface.0.clone();

    // INFO: --------------------------------------
    //         Set up the rendering context
    // --------------------------------------------

    let output_texture = match surface.get_current_texture() {
        Ok(texture) => texture,
        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
            warn!("Surface lost or outdated. Reconfiguring...");
            surface.configure(&device, &config.0);
            world.insert_resource(render_graph);
            return;
        }
        Err(e) => {
            error!("Error acquiring surface texture: {:?}", e);
            world.insert_resource(render_graph);
            return;
        }
    };

    let output_view = output_texture
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Rendergraph Encoder"),
    });

    // INFO: -----------------------------------
    //         Execute the render pipeline
    // -----------------------------------------

    render_graph.run(
        &mut RenderContext {
            device: &device,
            queue: &queue,
            encoder: &mut encoder,
            surface_texture_view: &output_view,
        },
        world,
    );

    queue.submit(std::iter::once(encoder.finish()));
    output_texture.present();

    // reset state to normal
    world.insert_resource(render_graph);
}
