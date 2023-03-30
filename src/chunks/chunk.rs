use std::{rc::Weak, sync::RwLock};
use block_mesh::ndshape::{ConstShape, ConstShape3u32};

use crate::prelude::*;

// A 16^3 chunk with 1-voxel boundary padding.
pub type ChunkShape = ConstShape3u32<{CHUNK_SIZE as u32 + 2}, {CHUNK_SIZE as u32 + 2}, {CHUNK_SIZE as u32 + 2}>;

pub struct Chunk {
    pub pos: IVec3,
    pub blocks: [Block; ChunkShape::SIZE as usize],
    pub dirty: bool,
    pub neighboars: [Weak<RwLock<Chunk>>; 6],
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            pos: IVec3::ZERO,
            blocks: [Block::default(); ChunkShape::SIZE as usize],
            dirty: false,
            neighboars: [
                Weak::new(),
                Weak::new(),
                Weak::new(),
                Weak::new(),
                Weak::new(),
                Weak::new(),
            ],
        }
    }
}

impl Chunk {
    pub fn chunk_from_pos(pos: IVec3) -> Self {
        Self {
            pos,
            blocks: block_generation(pos),
            dirty: false,
            neighboars: [
                Weak::new(),
                Weak::new(),
                Weak::new(),
                Weak::new(),
                Weak::new(),
                Weak::new(),
            ],
        }
    }

    pub fn get_world_cords(&self) -> Vec3 {
        Vec3::new (
            (self.pos.x * CHUNK_SIZE) as f32 ,
            (self.pos.y * CHUNK_SIZE) as f32,
            (self.pos.z * CHUNK_SIZE) as f32,
        )
    }
}