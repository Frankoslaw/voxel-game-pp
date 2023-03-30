#![allow(unused)]
use crate::prelude::*;

use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};


#[derive(Default, Clone, Copy, Eq, PartialEq)]
pub enum Block {
    #[default]
    Air,
    Dirt,
    Grass,
    Cobblestone
}

pub const EMPTY: Block = Block::Air;

impl Voxel for Block {
    fn get_visibility(&self) -> VoxelVisibility {
        if *self == EMPTY {
            VoxelVisibility::Empty
        } else {
            VoxelVisibility::Opaque
        }
    }
}

impl MergeVoxel for Block {
    type MergeValue = Self;

    fn merge_value(&self) -> Self::MergeValue {
        *self
    }
}

impl Block {
    pub fn get_face_index(&self, direction: VoxelDirection) -> u32 {
        match self {
            Block::Air => 0,
            Block::Grass => match direction {
                VoxelDirection::Top => 32 * 10 + 4,
                VoxelDirection::Bottom => 32 * 5 + 8,
                _ => 32 * 10 + 1
            },
            Block::Dirt => 32 * 5 + 8,
            Block::Cobblestone => 32 * 5 + 3,
        }
    }
}