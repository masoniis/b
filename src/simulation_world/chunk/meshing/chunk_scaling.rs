use crate::simulation_world::{
    block::block_registry::AIR_BLOCK_ID,
    chunk::{types::ChunkLod, ChunkBlocksComponent, CHUNK_SIDE_LENGTH},
};

/// Upsamples a low-LOD (e.g., 16x16x16) chunk to a high-LOD (e.g., 32x32x32) vec.
pub fn upsample_chunk(
    chunk_to_upsample: &ChunkBlocksComponent,
    target_lod: ChunkLod,
) -> ChunkBlocksComponent {
    let target_size = (CHUNK_SIDE_LENGTH >> *target_lod) as usize;
    let mut new_blocks = vec![AIR_BLOCK_ID; target_size * target_size * target_size];

    let lod_diff_shift = *chunk_to_upsample.lod() - *target_lod;

    if cfg!(debug_assertions) {
        if lod_diff_shift == 0 {
            panic!(
                "upsample_chunk: chunk LOD ({}) must be less than target LOD ({}).",
                chunk_to_upsample.lod(),
                target_lod
            );
        }
    }

    let z_shift = (target_size as u32).trailing_zeros() as u8;
    let x_shift = z_shift * 2;

    for y in 0..target_size {
        for z in 0..target_size {
            for x in 0..target_size {
                // corresponding block in the low-LOD chunk
                let lod_x = x >> lod_diff_shift;
                let lod_y = y >> lod_diff_shift;
                let lod_z = z >> lod_diff_shift;

                // write upsampled block
                let block_id = chunk_to_upsample.get_data_unchecked(lod_x, lod_y, lod_z);
                let index = (x << x_shift) | (z << z_shift) | y;

                if cfg!(debug_assertions) {
                    if index >= new_blocks.len() {
                        panic!(
                            "upsample_chunk: Calculated index ({}) out of bounds for target size ({}).",
                            index, target_size
                        );
                    }
                }

                new_blocks[index] = block_id;
            }
        }
    }

    ChunkBlocksComponent::new(target_lod, new_blocks)
}

/// Downsamples a high-LOD (e.g., 32x32x32) chunk to a low-LOD (e.g., 16x16x16) vec.
pub fn downsample_chunk(
    chunk_to_downsample: &ChunkBlocksComponent,
    target_lod: ChunkLod,
) -> ChunkBlocksComponent {
    let target_size = (CHUNK_SIDE_LENGTH >> *target_lod) as usize;
    let mut new_blocks = vec![AIR_BLOCK_ID; target_size * target_size * target_size];

    let lod_diff_shift = *target_lod - *chunk_to_downsample.lod();

    if cfg!(debug_assertions) {
        if lod_diff_shift == 0 {
            panic!(
                "downsample_chunk: target LOD ({}) must be greater than (lower detail) chunk LOD ({}).",
                target_lod,
                chunk_to_downsample.lod()
            );
        }
    }

    let z_shift = (target_size as u32).trailing_zeros();
    let x_shift = z_shift * 2;

    for y in 0..target_size {
        for z in 0..target_size {
            for x in 0..target_size {
                // for now, downsample by taking the first block in each region
                // later might add more advanced downsampling (e.g., averaging, majority, etc.)
                let source_x = x << lod_diff_shift;
                let source_y = y << lod_diff_shift;
                let source_z = z << lod_diff_shift;

                // write the downsampled block
                let block_id = chunk_to_downsample.get_data_unchecked(source_x, source_y, source_z);
                let index = (x << x_shift) | (z << z_shift) | y;

                if cfg!(debug_assertions) {
                    if index >= new_blocks.len() {
                        panic!(
                            "downsample_chunk: Calculated index ({}) out of bounds for target size ({}).",
                            index, target_size
                        );
                    }
                }

                new_blocks[index] = block_id;
            }
        }
    }
    ChunkBlocksComponent::new(target_lod, new_blocks)
}
