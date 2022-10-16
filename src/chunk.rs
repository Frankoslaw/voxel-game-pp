use std::cmp::{min, max};

use crate::{chunk_mesh_builder::ChunkMeshBuilder, voxel_constants::{CHUNK_SIZE, FACE_CHECKS}};
use bevy::prelude::Mesh;

pub struct Chunk {
    blocks: [[[bool; CHUNK_SIZE[2]]; CHUNK_SIZE[1]]; CHUNK_SIZE[0]],
    mesh_builder: ChunkMeshBuilder,
}

impl Chunk {
    pub fn new() -> Self {
        let mut blocks = [[[false; CHUNK_SIZE[2]]; CHUNK_SIZE[1]]; CHUNK_SIZE[0]];
        for x in 0..CHUNK_SIZE[0] {
            for y in 0..CHUNK_SIZE[1] {
                for z in 0..CHUNK_SIZE[2] {
                    blocks[x][y][z] = true;
                }
            }
        }

        Chunk {
            blocks,
            mesh_builder: ChunkMeshBuilder::new()
        }
    }

    pub fn check_voxel(&mut self, pos: [u32; 3]){
        for i in 0..6{
            let check_x = min(max(pos[0] as i32 + FACE_CHECKS[i][0], 0) as u32, (CHUNK_SIZE[0] - 1) as u32);
            let check_y = min(max(pos[1] as i32 + FACE_CHECKS[i][1], 0) as u32, (CHUNK_SIZE[1] - 1) as u32);
            let check_z = min(max(pos[2] as i32 + FACE_CHECKS[i][2], 0) as u32, (CHUNK_SIZE[2] - 1) as u32);

            if (check_x != pos[0] || check_y != pos[1] || check_z != pos[2]) &&
                self.blocks[check_x as usize][check_y as usize][check_z as usize] {
                continue;
            }

            self.mesh_builder.add_face(pos, i as u8);
        }
    }

    pub fn build_mesh(mut self) -> Mesh {
        for x in 0..CHUNK_SIZE[0] {
            for y in 0..CHUNK_SIZE[1] {
                for z in 0..CHUNK_SIZE[2] {
                    let pos = [x as u32, y as u32, z as u32];

                    self.check_voxel(pos);
                }
            }
        }

        self.mesh_builder.build()
    }
}