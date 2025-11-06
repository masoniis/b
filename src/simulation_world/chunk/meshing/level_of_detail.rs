use crate::simulation_world::chunk::CHUNK_HEIGHT;

pub const MAX_LOD: usize = {
    let mut res: usize = 0;
    let mut curr_height = CHUNK_HEIGHT;

    while curr_height > 0 {
        res += 1;
        curr_height /= 2;
    }

    res
};

// needs to take in a full sized chunk with padding and average it to half of each dimension
// to produce a new chunk object half the size that can then be meshed with fewer verts
