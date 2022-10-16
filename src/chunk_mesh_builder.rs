use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};
use crate::voxel_constants::*;

#[derive(Default)]
pub struct ChunkMeshBuilder {
    vertices: Vec<[f32; 3]>,
    triangles: Vec<u32>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    face_count:u32
}

impl ChunkMeshBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    fn add_vec3(mut base: [f32; 3], addition: [u32; 3]) -> [f32; 3] {
        for i in 0..3 {
            base[i] += addition[i] as f32;
        }
        base
    }

    pub fn add_face(&mut self, coord: [u32; 3], face_index: u8) {
        for i in &VERTICES[face_index as usize] {
            self.vertices.push(Self::add_vec3(*i, coord));
        }

        let mut arr=TRIANGLES.clone();
        self.triangles.extend_from_slice({
            for i in &mut arr {
                *i+=4*self.face_count;
            }
            &arr
        });

        for _ in 0..4 {
            self.normals.push(NORMALS[face_index as usize]);
        }

        self.uvs.extend_from_slice(&UVS);
        self.face_count+=1;
    }

    pub fn build(self)->Mesh{
        let mut msh=Mesh::new(PrimitiveTopology::TriangleList);
        msh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.vertices);
        msh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        msh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);

        msh.set_indices(Some(Indices::U32(self.triangles)));
        msh
    }
}