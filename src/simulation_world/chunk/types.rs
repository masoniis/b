use crate::prelude::*;
use crate::simulation_world::chunk::CHUNK_SIDE_LENGTH;
use std::fmt::{Display, Formatter};
use std::mem::MaybeUninit;
use std::sync::Arc;

/// A type-safe wrapper for a Level of Detail value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref, DerefMut)]
pub struct ChunkLod(pub u8);

impl Display for ChunkLod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ChunkLod {
    /// Returns the side length (e.g., 32, 16, 8) for this LOD.
    #[inline(always)]
    pub fn sidelength(self) -> usize {
        CHUNK_SIDE_LENGTH >> self.0
    }

    /// Returns the area (sidelength^2) for this LOD.
    #[inline(always)]
    pub fn sidelength_pow2(self) -> usize {
        let size = self.sidelength();
        size * size
    }

    /// Returns the volume (sidelength^3) for this LOD.
    #[inline(always)]
    pub fn sidelength_pow3(self) -> usize {
        let size = self.sidelength();
        size * size * size
    }

    /// Calculates the bit-shift required for Z indexing at this LOD.
    #[inline(always)]
    pub fn z_shift(self) -> u8 {
        self.sidelength().trailing_zeros() as u8
    }

    /// Calculates the bit-shift required for Y indexing at this LOD.
    #[inline(always)]
    pub fn y_shift(self) -> u8 {
        self.z_shift() * 2
    }
}

/// A temporary read-only view into a volume's data optimized for hot loops.
#[derive(Clone, Copy)]
pub struct VolumeDataView<'a, T> {
    data: &'a [T],
    x_shift: u8,
    z_shift: u8,
}

impl<'a, T: Copy> VolumeDataView<'a, T> {
    /// Gets a piece of data from the chunk volume.
    ///
    /// Caller must ensure x, y, z are within bounds or undefined behavior will occur.
    #[inline(always)]
    pub fn get_data(&self, x: usize, y: usize, z: usize) -> T {
        let index = (x << self.x_shift) | (z << self.z_shift) | y;

        if cfg!(debug_assertions) {
            let size = 1 << self.z_shift;
            if x >= size || y >= size || z >= size {
                error!(
                    "get_data: Out of bounds: ({}, {}, {}) in chunk size {}",
                    x, y, z, size
                );
            }
        }

        unsafe { *self.data.get_unchecked(index) }
    }
}

/// A temporary accessor for safe, high-speed batch writes to a chunk volume.
pub struct VolumeDataWriter<'a, T> {
    data: &'a mut [T],
    x_shift: u8,
    z_shift: u8,
}

impl<'a, T: Copy> VolumeDataWriter<'a, T> {
    /// Sets data in the chunk volume to a given value.
    ///
    /// Caller must ensure x, y, z are within bounds or undefined behavior will occur.
    #[inline(always)]
    pub fn set_data(&mut self, x: usize, y: usize, z: usize, data: T) {
        let index = (x << self.x_shift) | (z << self.z_shift) | y;

        if cfg!(debug_assertions) {
            let size = 1 << self.z_shift; // Calculate size from shift
            if x >= size || y >= size || z >= size {
                panic!(
                    "DenseDataAccessor::set_block: Out of bounds ({}, {}, {}) for size {}",
                    x, y, z, size
                );
            }
            // Also check index just in case logic is wrong
            if index >= self.data.len() {
                panic!(
                    "DenseDataAccessor: Calculated index {} is out of bounds {}",
                    index,
                    self.data.len()
                );
            }
        }

        unsafe {
            *self.data.get_unchecked_mut(index) = data;
        }
    }

    /// Sets a piece of data from the chunk volume by index.
    ///
    /// Caller must insure the index is within bounds or undefined behavior will occur.
    pub fn set_at_index(&mut self, index: usize, data: T) {
        unsafe { *self.data.get_unchecked_mut(index) = data };
    }

    /// Gets a piece of data from the chunk volume by coordinate.
    ///
    /// Caller must ensure x, y, z are within bounds or undefined behavior will occur.
    #[inline(always)]
    pub fn get_data(&self, x: usize, y: usize, z: usize) -> T {
        let index = (x << self.x_shift) | (z << self.z_shift) | y;

        if cfg!(debug_assertions) {
            let size = 1 << self.z_shift;
            if x >= size || y >= size || z >= size {
                error!(
                    "get_data: Out of bounds: ({}, {}, {}) in chunk size {}",
                    x, y, z, size
                );
            }
        }

        unsafe { *self.data.get_unchecked(index) }
    }

    /// Gets a piece of data from the chunk volume by index.
    ///
    /// Caller must insure the index is within bounds or undefined behavior will occur.
    #[inline(always)]
    pub fn get_at_index(&self, index: usize) -> T {
        unsafe { *self.data.get_unchecked(index) }
    }

    /// Bulk sets the entire volume to a value (memset).
    ///
    /// MUCH faster than setting data one by one in a loop.
    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        self.data.fill(value);
    }

    /// Copies data from a slice directly into this volume (memcpy).
    ///
    /// The source slice MUST be the same size as the chunk volume.
    pub fn copy_from_slice(&mut self, source: &[T])
    where
        T: Copy,
    {
        self.data.copy_from_slice(source);
    }
}

// INFO: ----------------------------------------
//         generic chunk volume container
// ----------------------------------------------

/// Generic, LOD-aware, 3D container for chunk voxel data.
#[derive(Clone)]
pub struct ChunkVolumeData<T: Send + Sync + 'static> {
    data: Arc<[T]>,

    /// The size of one edge (e.g., 32, 16, 8, ...).
    size: usize,
    /// The level of detail (0 = full detail, 1 = half size, etc.).
    lod: ChunkLod,
    /// Pre-calculated shift for X (e.g., log2(size) * 2).
    pub x_shift: u8,
    /// Pre-calculated shift for Z (e.g., log2(size)).
    pub z_shift: u8,
}

impl<T: Copy + Send + Sync + 'static> ChunkVolumeData<T> {
    /// Creates a new volume with all bits set to zero.
    pub fn new_zeroed(lod: ChunkLod) -> Self {
        let size = CHUNK_SIDE_LENGTH >> *lod;
        let num_elements = size.pow(3);

        // allocate uninitialized mem
        let mut uninit_arc: Arc<[MaybeUninit<T>]> = Arc::new_uninit_slice(num_elements);
        // get a raw mutable pointer to the data
        let ptr = Arc::get_mut(&mut uninit_arc).unwrap().as_mut_ptr();
        // memset to 0
        unsafe {
            std::ptr::write_bytes(ptr, 0, num_elements);
        }
        // freeze into an initialized Arc
        let data: Arc<[T]> = unsafe { uninit_arc.assume_init() };

        // calculate shifts
        let z_shift = size.trailing_zeros() as u8;
        let x_shift = (z_shift * 2) as u8;

        Self {
            data,
            size,
            lod,
            x_shift,
            z_shift,
        }
    }

    /// Creates a new volume component filled with `value`.
    pub fn new_filled(lod: ChunkLod, value: T) -> Self {
        let size = CHUNK_SIDE_LENGTH >> *lod;
        let num_elements = size.pow(3);

        // allocate uninitialized mem
        let mut uninit_arc: Arc<[MaybeUninit<T>]> = Arc::new_uninit_slice(num_elements);
        // get a raw mutable slice
        let uninit_slice = Arc::get_mut(&mut uninit_arc).unwrap();
        // fill memory
        for element in uninit_slice.iter_mut() {
            element.write(value);
        }
        // freeze into an initialized Arc
        let data: Arc<[T]> = unsafe { uninit_arc.assume_init() };

        // calculate shifts
        let z_shift = size.trailing_zeros() as u8;
        let x_shift = (z_shift * 2) as u8;

        Self {
            data,
            size,
            lod,
            x_shift,
            z_shift,
        }
    }

    /// Creates a new volume component from a vec `data` for a specific `lod`.
    ///
    /// Panics if the length of `data` does not match the expected size.
    pub fn from_vec(lod: ChunkLod, data: Vec<T>) -> Self {
        let size = CHUNK_SIDE_LENGTH >> *lod;
        let expected_len = size.pow(3);

        if data.len() != expected_len {
            panic!(
                "ChunkVolumeData::new: Data length ({}) does not match expected length ({}) for LOD {} (size {}).",
                data.len(),
                expected_len,
                lod,
                size
            );
        }

        let z_shift = size.trailing_zeros() as u8;
        let x_shift = (z_shift * 2) as u8;

        Self {
            data: data.into_boxed_slice().into(),
            size,
            lod,
            x_shift,
            z_shift,
        }
    }

    /// Returns the size of one edge of the chunk volume.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the level of detail of the chunk volume.
    pub fn lod(&self) -> ChunkLod {
        self.lod
    }

    /// Returns a mutable reference to the underlying Vec, cloning if it's shared.
    #[inline(always)]
    pub fn get_data_mut(&mut self) -> &mut [T] {
        Arc::make_mut(&mut self.data)
    }

    /// Returns an immutable view of the underlying volume data.
    #[inline(always)]
    pub fn get_data_view(&self) -> VolumeDataView<'_, T> {
        VolumeDataView {
            data: &self.data,
            x_shift: self.x_shift,
            z_shift: self.z_shift,
        }
    }

    /// Returns a mutable accessor to the underlying volume data.
    #[inline(always)]
    pub fn get_data_writer(&mut self) -> VolumeDataWriter<'_, T> {
        VolumeDataWriter {
            data: Arc::make_mut(&mut self.data),
            x_shift: self.x_shift,
            z_shift: self.z_shift,
        }
    }

    /// Gets the data at the given local coordinates.
    ///
    /// Has undefined behavior if called on indices out of chunk bounds.
    #[inline(always)]
    pub fn get_data_unchecked(&self, x: usize, y: usize, z: usize) -> T {
        let index = (x << self.x_shift) | (z << self.z_shift) | y;

        if cfg!(debug_assertions) && (x >= self.size || y >= self.size || z >= self.size) {
            error!(
                "get_data_unchecked: Attempted to access voxel data out of bounds: ({}, {}, {}) in a chunk of size {}",
                x, y, z, self.size
            );
        }

        unsafe { *self.data.get_unchecked(index) }
    }
}

// INFO: ----------------------------------------
//         generic chunk column container
// ----------------------------------------------

/// Generic, LOD-aware, 2D container for chunk column data (e.g., climate, heightmaps).
#[derive(Clone)]
pub struct ChunkColumnData<T: Send + Sync + 'static> {
    data: Arc<Vec<T>>,

    /// The size of one edge (e.g., 32, 16, 8, ...).
    size: usize,
    /// The level of detail (0 = full detail, 1 = half size, etc.).
    lod: ChunkLod,
    /// Pre-calculated shift for Z (e.g., log2(size)).
    z_shift: u32,
}

impl<T: Copy + Send + Sync + 'static> ChunkColumnData<T> {
    /// Creates a new column component filled with `data` for a specific `lod`.
    ///
    /// Panics if the length of `data` does not match the expected area for the
    /// given `lod`.
    pub fn new(lod: ChunkLod, data: Vec<T>) -> Self {
        let size = CHUNK_SIDE_LENGTH >> *lod;
        let expected_len = size.pow(2); // 2D area

        if data.len() != expected_len {
            panic!(
                "ChunkColumnData::new: Data length ({}) does not match expected length ({}) for LOD {} (size {}x{}).",
                data.len(),
                expected_len,
                lod,
                size,
                size
            );
        }

        let z_shift = size.trailing_zeros();

        Self {
            data: Arc::new(data),
            size,
            lod,
            z_shift,
        }
    }

    /// Creates a new column component filled with a `default_value`.
    pub fn new_filled(lod: ChunkLod, default_value: T) -> Self {
        let size = CHUNK_SIDE_LENGTH >> *lod;
        let num_elements = size.pow(2);
        let data = vec![default_value; num_elements];
        Self::new(lod, data)
    }

    /// Returns the size of one edge of the chunk column.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the level of detail of the chunk column.
    pub fn lod(&self) -> ChunkLod {
        self.lod
    }

    /// Gets the data at the given local coordinates.
    ///
    /// This is infallible but will return an incorrect value on out-of-bounds.
    #[inline(always)]
    pub fn get_data_unchecked(&self, x: usize, z: usize) -> T {
        if cfg!(debug_assertions) && (x >= self.size || z >= self.size) {
            error!(
                "get_data_unchecked: Attempted to access column data out of bounds: ({}, {}) in a chunk of size {}",
                x, z, self.size
            );
        }

        let index = (z << self.z_shift) | x;
        unsafe { *self.data.get_unchecked(index) }
    }

    /// Sets the data at the given local coordinates.
    #[inline(always)]
    pub fn set_data(&mut self, x: usize, z: usize, item: T) {
        if cfg!(debug_assertions) && (x >= self.size || z >= self.size) {
            error!(
                "set_data: Attempted to set column data out of bounds: ({}, {}) in a chunk of size {}",
                x, z, self.size
            );
            return;
        }

        let index = (z << self.z_shift) | x;
        Arc::make_mut(&mut self.data)[index] = item;
    }
}

// INFO: -----------------------------------
//         voxel world-pos iterators
// -----------------------------------------

/// An iterator that yields the local and world coordinates for every
/// voxel within a chunk at a specific LOD.
pub struct WorldVoxelPositionIterator {
    base_world_pos: IVec3,
    size: usize,
    step: i32,

    // current iterator state
    x: usize,
    y: usize,
    z: usize,
}

impl WorldVoxelPositionIterator {
    /// Creates a new iterator for a chunk at `coord` and `lod`.
    pub fn new(coord: IVec3, lod: ChunkLod) -> Self {
        let size = lod.sidelength();
        let step = 1i32 << lod.0;
        let base_world_pos = coord * CHUNK_SIDE_LENGTH as i32;

        Self {
            base_world_pos,
            size,
            step,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

/// The item yielded by the WorldVoxelIterator.
pub struct VoxelPos {
    /// The local voxel coordinate (e.g., 0..31) at this LOD.
    pub local: (usize, usize, usize),
    /// The corresponding world-space coordinate.
    pub world: IVec3,
}

impl Iterator for WorldVoxelPositionIterator {
    type Item = VoxelPos;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.size {
            return None; // finished iterating
        }

        let local_pos = (self.x, self.y, self.z);

        // calculate world pos by scaling the local pos and adding the base
        let world_pos = self.base_world_pos
            + (IVec3::new(self.x as i32, self.y as i32, self.z as i32) * self.step);

        let item = VoxelPos {
            local: local_pos,
            world: world_pos,
        };

        // advance state (y -> z -> x)
        //
        // this order is intentional to maintain
        // cache locality for `ChunkVolumeData`
        self.y += 1;
        if self.y >= self.size {
            self.y = 0;
            self.z += 1;
            if self.z >= self.size {
                self.z = 0;
                self.x += 1;
            }
        }

        Some(item)
    }
}

/// An iterator that yields voxel positions in X-major, Z-medial, Y-innermost order.
///
/// It also signals when it enters a new (x, z) column.
pub struct WorldVoxelIteratorWithColumn {
    base_world_pos: IVec3,
    size: usize,
    step: i32,

    // Current iterator state (x, z, y)
    x: usize,
    y: usize,
    z: usize,
}

impl WorldVoxelIteratorWithColumn {
    /// Creates a new iterator for a chunk at `coord` and `lod`.
    pub fn new(coord: IVec3, lod: ChunkLod) -> Self {
        let size = lod.sidelength();
        let step = 1i32 << lod.0;
        let base_world_pos = coord * CHUNK_SIDE_LENGTH as i32;

        Self {
            base_world_pos,
            size,
            step,
            x: 0,
            y: 0,
            z: 0,
        }
    }
}

/// The item yielded by the BiomeGenPositionIterator.
pub struct VoxelPositionWithColumn {
    /// The local voxel coordinate (x, y, z).
    pub local: (usize, usize, usize),
    /// The corresponding world-space coordinate.
    pub world: IVec3,
    /// True if this is the first `y` (y=0) for a new (x, z) column.
    pub is_new_column: bool,
}

impl Iterator for WorldVoxelIteratorWithColumn {
    type Item = VoxelPositionWithColumn;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.size {
            return None; // done iterating
        }

        let local_pos = (self.x, self.y, self.z);

        // calculate world pos by scaling the local pos and adding the base
        let world_pos = self.base_world_pos
            + (IVec3::new(self.x as i32, self.y as i32, self.z as i32) * self.step);

        let item = VoxelPositionWithColumn {
            local: local_pos,
            world: world_pos,
            is_new_column: (self.y == 0),
        };

        // advance state (y -> z -> x)
        //
        // this order is intentional to maintain
        // cache locality for `ChunkVolumeData`
        self.y += 1;
        if self.y >= self.size {
            self.y = 0;
            self.z += 1;
            if self.z >= self.size {
                self.z = 0;
                self.x += 1;
            }
        }

        Some(item)
    }
}

/// An iterator that yields the (x, z) columns for a chunk.
///
/// Iteration order is X-major, Z-medial (e.g., (0,0), (0,1), ..., (1,0), (1,1), ...).
/// This is intended for O(n^2) terrain painting passes.
pub struct WorldColumnIterator {
    base_world_pos: IVec3,
    size: usize,
    step: i32,

    // current iterator state
    x: usize,
    z: usize,
}

impl WorldColumnIterator {
    /// Creates a new iterator for a chunk at `coord` and `lod`.
    pub fn new(coord: IVec3, lod: ChunkLod) -> Self {
        let size = lod.sidelength();
        let step = 1i32 << lod.0;
        let base_world_pos = coord * CHUNK_SIDE_LENGTH as i32;

        Self {
            base_world_pos,
            size,
            step,
            x: 0,
            z: 0,
        }
    }

    /// Get the size (sidelength) of the chunk.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the world-space step size for this LOD.
    pub fn step(&self) -> i32 {
        self.step
    }

    /// Get the base world-space Y coordinate for this chunk.
    pub fn base_world_y(&self) -> i32 {
        self.base_world_pos.y
    }
}

/// The item yielded by the WorldColumnIterator.
pub struct ColumnPos {
    /// The local column coordinate (e.g., 0..31) at this LOD.
    pub local: (usize, usize), // (x, z)
    /// The corresponding world-space XZ coordinate.
    pub world_xz: (i32, i32), // (world_x, world_z)
}

impl Iterator for WorldColumnIterator {
    type Item = ColumnPos;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.size {
            return None; // finished iterating
        }

        let local_pos = (self.x, self.z);

        // calculate world xz pos
        let world_x = self.base_world_pos.x + (self.x as i32 * self.step);
        let world_z = self.base_world_pos.z + (self.z as i32 * self.step);

        let item = ColumnPos {
            local: local_pos,
            world_xz: (world_x, world_z),
        };

        // advance state (z -> x)
        self.z += 1;
        if self.z >= self.size {
            self.z = 0;
            self.x += 1;
        }

        Some(item)
    }
}
