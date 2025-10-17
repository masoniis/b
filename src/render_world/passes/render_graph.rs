use super::main_pass::render::RenderPassNode;
use super::ui_pass::render::UiPassNode;
use crate::prelude::*;
use crate::render_world::resources::GraphicsContextResource;
use bevy_ecs::prelude::*;
use std::any::TypeId;
use std::collections::HashMap;

// INFO: ---------------------------------------------
//         Rendergraph trait, types, and logic
// ---------------------------------------------------

/// A trait for a node in the render graph.
pub trait RenderNode: Send + Sync + 'static {
    fn run(&mut self, render_context: &mut RenderContext, world: &World);
}

/// A context passed to each render node when it is executed.
pub struct RenderContext<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub surface_texture_view: &'a wgpu::TextureView,
}

/// A container for a render graph node to supplement a node with metadata.
pub struct RenderGraphNode {
    pub node: Box<dyn RenderNode>,
    pub name: &'static str,
    pub is_active: bool,
}

/// A directed acyclic graph (DAG) of render nodes that can have dependencies on each other.
#[derive(Resource, Default)]
pub struct RenderGraph {
    nodes: HashMap<TypeId, RenderGraphNode>,
    edges: HashMap<TypeId, Vec<TypeId>>,
}

impl RenderGraph {
    /// Add a node to the graph. All nodes in the graph **will** be executed.
    pub fn add_node<L: 'static, N: RenderNode>(
        &mut self,
        name: &'static str,
        node: N,
        is_active: bool,
    ) {
        let label_id = TypeId::of::<L>();
        self.nodes.insert(
            label_id,
            RenderGraphNode {
                node: Box::new(node),
                name,
                is_active,
            },
        );
    }

    /// Toggle a node's active state. Inactive nodes will not run (but dependencies will for now)
    pub fn set_active<L: 'static>(&mut self, is_active: bool) {
        let label_id = TypeId::of::<L>();
        if let Some(entry) = self.nodes.get_mut(&label_id) {
            entry.is_active = is_active;
        } else {
            warn!("Render graph node with id {:?} not found!", label_id);
        }
    }

    /// Adds a dependency such that the first node parameter depends on the second node.
    ///
    /// Such a dependency ensures that the first node will only run AFTER the second node.
    pub fn add_node_dependency<L: 'static, Dep: 'static>(&mut self) {
        let node_id = TypeId::of::<L>();
        let dependency_id = TypeId::of::<Dep>();
        self.edges.entry(node_id).or_default().push(dependency_id);
    }

    /// Execute the graph, abiding by all dependencies.
    pub fn run(&mut self, render_context: &mut RenderContext, world: &World) {
        // Collect keys so we can borrow mutably in the loop
        let node_names: Vec<_> = self.nodes.keys().copied().collect();

        let mut visited = HashMap::new();
        for &node_name in &node_names {
            self.run_node(&node_name, render_context, world, &mut visited);
        }
    }

    fn run_node(
        &mut self,
        node_id: &TypeId,
        render_context: &mut RenderContext,
        world: &World,
        visited: &mut HashMap<TypeId, bool>,
    ) {
        if let Some(entry) = self.nodes.get(node_id) {
            if !entry.is_active {
                return; // don't run inactive nodes
            }
        } else {
            error!("Render graph node with id {:?} not found!", node_id);
            return;
        }

        if let Some(&true) = visited.get(node_id) {
            return; // already visited
        }

        visited.insert(*node_id, true);

        if let Some(dependencies) = self.edges.get(node_id) {
            let deps_to_run = dependencies.clone();
            for &dep in &deps_to_run {
                self.run_node(&dep, render_context, world, visited);
            }
        }

        if let Some(render_node) = self.nodes.get_mut(node_id) {
            render_node.node.run(render_context, world);
        }
    }
}

// INFO: --------------------------------------------------------
//         Systems to set up and execute the render graph
// --------------------------------------------------------------

/// An exclusive system that runs once at startup to create, configure,
/// and insert the application's RenderGraph resource.
pub fn setup_render_graph(world: &mut World) {
    let mut render_graph = RenderGraph::default();

    let main_pass_node = RenderPassNode::new(world);
    let ui_pass_node = UiPassNode;

    render_graph.add_node::<RenderPassNode, _>("MainPass3d", main_pass_node, true);
    render_graph.add_node::<UiPassNode, _>("UiPass", ui_pass_node, true);
    render_graph.add_node_dependency::<UiPassNode, RenderPassNode>();

    world.insert_resource(render_graph);

    info!("Render graph created and configured!");
}

/// A system to run the configured render graph each frame.
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
