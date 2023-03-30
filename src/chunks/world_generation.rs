use bevy::prelude::IVec3;
use block_mesh::ndshape::ConstShape;
use noise::{Perlin, NoiseFn};

use crate::prelude::*;

pub fn block_generation(pos: IVec3, seed: u32) -> [Block; ChunkShape::SIZE as usize]{
    let mut voxels = [EMPTY; ChunkShape::SIZE as usize];
    let noise_fn = Perlin::new(seed);

    for x in 1..ChunkShape::ARRAY[0] - 1 {
        for z in 1..ChunkShape::ARRAY[2] - 1 {
            let terrain_h = noise_fn.get([
                (x as f64) / ChunkShape::ARRAY[0] as f64 + pos.x as f64,
                (z as f64 ) / ChunkShape::ARRAY[2] as f64 + pos.z as f64
            ]) * 5. + 3.;


            for y in 1..ChunkShape::ARRAY[1] - 1 {
                let i = ChunkShape::linearize([x, y, z]);

                voxels[i as usize] = if y as i32 + (pos.y * CHUNK_SIZE) < terrain_h as i32 {
                    Block::Grass
                }else{
                    EMPTY
                };
            }
        }
    }

    voxels
}