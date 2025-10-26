use std::hash::{Hash, Hasher};

/// A type to represent a vertex with position and color for the gpu
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub texture_index: u32,
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 5] = wgpu::vertex_attr_array![
        0 => Float32x3, // position
        1 => Float32x3, // normal
        2 => Float32x3, // color
        3 => Float32x2, // tex_coords
        4 => Uint32,    // texture_index
    ];

    pub fn new(
        position: [f32; 3],
        normal: [f32; 3],
        tex_coords: [f32; 2],
        texture_index: u32,
    ) -> Self {
        Self {
            position,
            color: [1.0, 1.0, 1.0],
            normal,
            tex_coords,
            texture_index,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.position
            .iter()
            .zip(other.position.iter())
            .all(|(a, b)| a.to_bits() == b.to_bits())
            && self
                .normal
                .iter()
                .zip(other.normal.iter())
                .all(|(a, b)| a.to_bits() == b.to_bits())
            && self
                .color
                .iter()
                .zip(other.color.iter())
                .all(|(a, b)| a.to_bits() == b.to_bits())
            && self
                .tex_coords
                .iter()
                .zip(other.tex_coords.iter())
                .all(|(a, b)| a.to_bits() == b.to_bits())
            && self.texture_index == other.texture_index
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.iter().for_each(|f| f.to_bits().hash(state));
        self.color.iter().for_each(|f| f.to_bits().hash(state));
        self.tex_coords.iter().for_each(|f| f.to_bits().hash(state));
        self.texture_index.hash(state);
    }
}
