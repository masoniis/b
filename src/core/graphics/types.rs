use std::hash::{Hash, Hasher};

/// A type to connect ECS components to the webgpu renderer
pub struct GpuMesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

/// A type to represent a vertex with position and color
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position
            .iter()
            .zip(other.position.iter())
            .all(|(a, b)| a.to_bits() == b.to_bits())
            && self
                .color
                .iter()
                .zip(other.color.iter())
                .all(|(a, b)| a.to_bits() == b.to_bits())
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.iter().for_each(|f| f.to_bits().hash(state));
        self.color.iter().for_each(|f| f.to_bits().hash(state));
    }
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x3, // color
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
