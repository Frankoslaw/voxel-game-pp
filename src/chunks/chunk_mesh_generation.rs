use bevy::render::mesh::{VertexAttributeValues, Indices};
use bevy::render::render_resource::PrimitiveTopology;
use block_mesh::ndshape::ConstShape;
use block_mesh::{greedy_quads, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};


use crate::prelude::*;

pub fn chunk_mesh_generation(
    chunk: &Chunk
) -> Mesh {
    let mut buffer = GreedyQuadsBuffer::new(chunk.blocks.len());
    let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;
    
    greedy_quads(
        &chunk.blocks,
        &ChunkShape {},
        [0; 3],
        [17; 3],
        &RIGHT_HANDED_Y_UP_CONFIG.faces,
        &mut buffer
    );


    let num_indices = buffer.quads.num_quads();
    let num_vertices = buffer.quads.num_quads() * 4;
    let mut indices = Vec::with_capacity(num_indices);
    let mut positions = Vec::with_capacity(num_vertices);
    let mut tex_coords = Vec::with_capacity(num_vertices);
    let mut texture_i = Vec::with_capacity(num_vertices);

    for (group, face) in buffer.quads.groups.into_iter().zip(faces.into_iter()) {
        for quad in group.into_iter() {
            indices.extend_from_slice(&face.quad_mesh_indices(positions.len() as u32));
            positions.extend_from_slice(&face.quad_mesh_positions(&quad.into(), 1.0));
            tex_coords.extend_from_slice(&face.tex_coords(
                RIGHT_HANDED_Y_UP_CONFIG.u_flip_face,
                true,
                &quad,
            ));


            texture_i.extend_from_slice(&[
                chunk.blocks[ChunkShape::linearize(quad.minimum) as usize]
                    .get_face_index(VoxelDirection::from_ivec(face.signed_normal()))
            ; 4]);
        }
    }

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(positions),
    );
    render_mesh.insert_attribute(
        CUSTOM_UV,
        VertexAttributeValues::Float32x2(tex_coords),
    );
    render_mesh.insert_attribute(
        ATTRIBUTE_TEXTURE_INDEX, 
        VertexAttributeValues::Uint32(texture_i)
    );
    render_mesh.set_indices(Some(Indices::U32(indices.clone())));

    render_mesh
}