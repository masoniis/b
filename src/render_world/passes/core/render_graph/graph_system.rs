use crate::prelude::*;
use crate::render_world::graphics_context::resources::{
    RenderDevice, RenderQueue, RenderSurface, RenderSurfaceConfig,
};
use crate::render_world::passes::core::{RenderContext, RenderGraph};
use crate::render_world::passes::opaque_pass::render::OpaquePassRenderNode;
use crate::render_world::passes::ui_pass::render::UiPassNode;
use bevy_ecs::prelude::*;

// INFO: --------------------------------------------------------
//         Systems to set up and execute the render graph
// --------------------------------------------------------------

/// An exclusive system that runs once at startup to create, configure,
/// and insert the application's RenderGraph resource.
pub fn setup_render_graph(world: &mut World) {
    let mut render_graph = RenderGraph::default();

    let main_pass_node = OpaquePassRenderNode::new(world);
    let ui_pass_node = UiPassNode;

    render_graph.add_node::<OpaquePassRenderNode, _>("MainPass3d", main_pass_node, true);
    render_graph.add_node::<UiPassNode, _>("UiPass", ui_pass_node, true);
    render_graph.add_node_dependency::<UiPassNode, OpaquePassRenderNode>();

    world.insert_resource(render_graph);

    info!("Render graph created and configured!");
}

#[instrument(skip_all)]
pub fn render_graph_system(world: &mut World) {
    // Take ownership of the graph
    let Some(mut render_graph) = world.remove_resource::<RenderGraph>() else {
        return;
    };

    // --- Get all required granular resources ---
    // We use .get_resource() on the world.
    let Some(device) = world.get_resource::<RenderDevice>() else {
        world.insert_resource(render_graph); // Put graph back
        return;
    };
    let Some(queue) = world.get_resource::<RenderQueue>() else {
        world.insert_resource(render_graph); // Put graph back
        return;
    };
    let Some(surface) = world.get_resource::<RenderSurface>() else {
        world.insert_resource(render_graph); // Put graph back
        return;
    };
    let Some(config) = world.get_resource::<RenderSurfaceConfig>() else {
        world.insert_resource(render_graph); // Put graph back
        return;
    };

    // Clone the Arcs to satisfy lifetimes
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
            // Pass the inner config data
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
        world, // Pass the world here
    );

    queue.submit(std::iter::once(encoder.finish()));
    output_texture.present();

    // Put the graph back when we're done
    world.insert_resource(render_graph);
}
