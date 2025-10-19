// WARNING: these constants rely on bitwise operations.
// DO NOT set them to anything that isn't a power of 2.
pub const CHUNK_WIDTH: usize = 32;
pub const CHUNK_HEIGHT: usize = 32;
pub const CHUNK_DEPTH: usize = 32;

// INFO: ------------------------
//         derived consts
// ------------------------------

// Z_SHIFT is log2(CHUNK_WIDTH)
pub const Z_SHIFT: usize = CHUNK_WIDTH.trailing_zeros() as usize;

// Y_SHIFT is log2(CHUNK_WIDTH * CHUNK_DEPTH)
pub const Y_SHIFT: usize = (CHUNK_WIDTH * CHUNK_DEPTH).trailing_zeros() as usize;

// Helper consts for indexing
pub const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH;
pub const INDEX_MASK: usize = CHUNK_SIZE - 1;
