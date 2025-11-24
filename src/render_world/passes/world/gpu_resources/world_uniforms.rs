use crate::prelude::*;
use crate::render_world::graphics_context::resources::RenderDevice;
use crate::render_world::types::PackedFace;
use bevy_ecs::prelude::*;
use offset_allocator::{Allocation, Allocator};

/// 128 MB = Enough for ~30,000 average chunks (4KB each)
const MEGA_BUFFER_SIZE: u64 = 128 * 1024 * 1024;
/// The max number of chunks for storing metadata
const MAX_CHUNKS: u64 = 10_000;

// INFO: --------------------
//         data types
// --------------------------

/// The raw data written to the Metadata Buffer.
/// Represents "Where is this chunk in the world" + "Where is its geometry".
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ChunkRenderData {
    pub world_pos: [f32; 3], // 12 bytes
    pub start_index: u32,    // 4 bytes (Total 16 bytes, perfectly aligned)
}

/// The Handle returned to the ECS.
/// Holds the tickets for both the Geometry memory and the Metadata slot.
#[derive(Clone, Copy)]
pub struct VoxelMesh {
    /// Ticket for the variable-sized geometry buffer
    pub geometry_allocation: Allocation,
    /// Ticket for the fixed-size metadata buffer (instance index)
    pub slot_index: u32,
    /// Number of faces to draw
    pub face_count: u32,
}

// INFO: --------------------------
//         internal helpers
// --------------------------------

/// A simple Free-List allocator for fixed-size slots (0..N).
struct SlotAllocator {
    free_indices: Vec<u32>,
    next_index: u32,
    capacity: u32,
}

impl SlotAllocator {
    fn new(capacity: u32) -> Self {
        Self {
            free_indices: Vec::new(),
            next_index: 0,
            capacity,
        }
    }

    fn allocate(&mut self) -> Option<u32> {
        // 1. Recycle used slot
        if let Some(idx) = self.free_indices.pop() {
            return Some(idx);
        }
        // 2. Mint new slot
        if self.next_index < self.capacity {
            let idx = self.next_index;
            self.next_index += 1;
            return Some(idx);
        }
        // 3. Full
        None
    }

    fn free(&mut self, index: u32) {
        self.free_indices.push(index);
    }
}

// INFO: ---------------------
//         the manager
// ---------------------------

/// The view bind group layout resource shared by all camera-centric render passes.
#[derive(Resource)]
pub struct ChunkStorageBindGroupLayout(pub wgpu::BindGroupLayout);

impl FromWorld for ChunkStorageBindGroupLayout {
    #[instrument(skip_all)]
    fn from_world(world: &mut World) -> Self {
        let device = world.resource::<RenderDevice>().clone();

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Chunk Storage Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, // chunk metadata
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1, // faces
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        Self(layout)
    }
}

#[derive(Resource)]
pub struct ChunkStorageManager {
    /// The Bind Group used in the Render Pass (Group 3)
    pub bind_group: wgpu::BindGroup,

    // GPU Buffers (Private to ensure data integrity via API)
    meta_buffer: wgpu::Buffer,
    face_buffer: wgpu::Buffer,

    // CPU Allocators (Private)
    geometry_allocator: Allocator,
    slot_allocator: SlotAllocator,
}

impl FromWorld for ChunkStorageManager {
    fn from_world(world: &mut World) -> Self {
        // 1. Get Dependencies
        let device = world.resource::<RenderDevice>();
        // We retrieve the layout we created in the Layout Setup System
        let layout = world.resource::<ChunkStorageBindGroupLayout>();

        // 2. Create Buffers
        let face_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Global Voxel SSBO"),
            size: MEGA_BUFFER_SIZE,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let meta_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Global Meta SSBO"),
            size: MAX_CHUNKS * std::mem::size_of::<ChunkRenderData>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // 3. Create Bind Group using the Shared Layout
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Global Voxel Bind Group"),
            layout: &layout.0,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0, // chunk metadata
                    resource: meta_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1, // chunk faces
                    resource: face_buffer.as_entire_binding(),
                },
            ],
        });

        // 4. Initialize Allocators
        Self {
            face_buffer,
            meta_buffer,
            bind_group,
            geometry_allocator: Allocator::new(MEGA_BUFFER_SIZE as u32),
            slot_allocator: SlotAllocator::new(MAX_CHUNKS as u32),
        }
    }
}

impl ChunkStorageManager {
    /// The unified API to upload a chunk.
    /// Handles finding space for geometry, finding a metadata slot,
    /// uploading bytes, and returning a safe Handle.
    pub fn allocate_chunk(
        &mut self,
        queue: &wgpu::Queue,
        faces: &[PackedFace],
        world_pos: [f32; 3],
    ) -> Option<VoxelMesh> {
        if faces.is_empty() {
            return None;
        }

        // 1. Allocate Geometry (Variable size)
        let size_bytes = (faces.len() * std::mem::size_of::<PackedFace>()) as u32;
        let geo_alloc = self.geometry_allocator.allocate(size_bytes)?;

        // 2. Allocate Slot (Fixed size)
        let slot_index = match self.slot_allocator.allocate() {
            Some(s) => s,
            None => {
                // Rollback: If we can't find a slot, free the geometry we just took!
                self.geometry_allocator.free(geo_alloc);
                return None;
            }
        };

        // 3. Upload Geometry Data
        queue.write_buffer(
            &self.face_buffer,
            geo_alloc.offset as u64,
            bytemuck::cast_slice(faces),
        );

        // 4. Upload Metadata
        // Convert byte offset to index offset (bytes / 4)
        let start_index = geo_alloc.offset / 4;

        let meta_data = ChunkRenderData {
            world_pos,
            start_index,
        };

        let meta_offset = slot_index as u64 * std::mem::size_of::<ChunkRenderData>() as u64;
        queue.write_buffer(
            &self.meta_buffer,
            meta_offset,
            bytemuck::bytes_of(&meta_data),
        );

        // 5. Return Handle
        Some(VoxelMesh {
            geometry_allocation: geo_alloc,
            slot_index,
            face_count: faces.len() as u32,
        })
    }

    /// Frees all resources associated with the chunk handle.
    pub fn free_chunk(&mut self, mesh: VoxelMesh) {
        self.geometry_allocator.free(mesh.geometry_allocation);
        self.slot_allocator.free(mesh.slot_index);
    }
}
