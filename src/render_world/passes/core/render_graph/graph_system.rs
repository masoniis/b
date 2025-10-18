use crate::prelude::*;
use crate::render_world::passes::core::{RenderContext, RenderGraph};
use crate::render_world::passes::opaque_pass::render::OpaquePassRenderNode;
use crate::render_world::passes::ui_pass::render::UiPassNode;
use crate::render_world::resources::GraphicsContextResource;
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

/// A system to run the configured render graph each frame.
#[instrument(skip_all)]
pub fn render_graph_system(world: &mut World) {
    let Some(mut render_graph) = world.remove_resource::<RenderGraph>() else {
        return;
    };
    let Some(gfx_resource) = world.get_resource::<GraphicsContextResource>() else {
        world.insert_resource(render_graph); // re-add the graph if we can't get gfx
        return;
    };

    // INFO: --------------------------------------
    //         Set up the rendering context
    // --------------------------------------------

    let gfx = &gfx_resource.context;
    let output_texture = match gfx.surface.get_current_texture() {
        Ok(texture) => texture,
        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
            warn!("Surface lost or outdated. Reconfiguring...");
            gfx.surface.configure(&gfx.device, &gfx.config);
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

    let mut encoder = gfx
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Rendergraph Encoder"),
        });

    // INFO: -----------------------------------
    //       Execute the render pipeline
    // -----------------------------------------

    render_graph.run(
        &mut RenderContext {
            device: &gfx.device,
            queue: &gfx.queue,
            encoder: &mut encoder,
            surface_texture_view: &output_view,
        },
        world,
    );

    gfx.queue.submit(std::iter::once(encoder.finish()));
    output_texture.present();

    world.insert_resource(render_graph);
}
