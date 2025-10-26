// INFO: ----------------------
//         chunk sizing
// ----------------------------

// the number of bitshifts to apply to the chunk_width and height
//
// 2^CHUNK_DIM_SHIFT will be the size of each side of the chunks
const CHUNK_DIM_SHIFT: usize = 4;

pub const CHUNK_WIDTH: usize = 2 << CHUNK_DIM_SHIFT;
pub const CHUNK_HEIGHT: usize = 2 << CHUNK_DIM_SHIFT;
pub const CHUNK_DEPTH: usize = 2 << CHUNK_DIM_SHIFT;

// INFO: -------------------------
//         render distance
// -------------------------------

/// The distance in the xz plane, in chunks, to load around the camera.
pub const RENDER_DISTANCE: i32 = 11;
/// The size of the vertical column that we render (chunks above/below will never be generated)
pub const WORLD_MIN_Y_CHUNK: i32 = 0;
pub const WORLD_MAX_Y_CHUNK: i32 = 256 >> (CHUNK_DIM_SHIFT + 1);

// INFO: ------------------------------
//         other derived consts
// ------------------------------------

// Z_SHIFT is log2(CHUNK_WIDTH)
pub const Z_SHIFT: usize = CHUNK_WIDTH.trailing_zeros() as usize;
// Y_SHIFT is log2(CHUNK_WIDTH * CHUNK_DEPTH)
pub const Y_SHIFT: usize = (CHUNK_WIDTH * CHUNK_DEPTH).trailing_zeros() as usize;

// Helper consts for indexing
pub const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_HEIGHT * CHUNK_DEPTH;
pub const CHUNK_SURFACE_SIZE: usize = CHUNK_WIDTH * CHUNK_DEPTH;
pub const INDEX_MASK: usize = CHUNK_SIZE - 1;
