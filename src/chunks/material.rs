use bevy::{
    reflect::TypeUuid,
    render::{render_resource::{AsBindGroup, ShaderRef, VertexFormat, RenderPipelineDescriptor, SpecializedMeshPipelineError}, mesh::{MeshVertexAttribute, MeshVertexBufferLayout}}, pbr::{MaterialPipeline, MaterialPipelineKey},
};


use crate::prelude::*;

#[derive(Resource)]
pub struct ChunkTexture(pub Handle<Image>);

pub fn load_chunk_texture(mut commands: Commands, server: Res<AssetServer>) {
    commands.insert_resource(ChunkTexture(server.load("textures/minecraft.png")));
}

pub const ATTRIBUTE_TEXTURE_INDEX: MeshVertexAttribute =
    MeshVertexAttribute::new("TextureIndex", 4294967293, VertexFormat::Uint32);

pub const CUSTOM_UV: MeshVertexAttribute = MeshVertexAttribute::new("CustomUV", 4294967294, VertexFormat::Float32x2);

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "642b56d7-f1bf-4fd6-be25-6fed49891480"]
pub struct ChunkMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub textures: Handle<Image>,
}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/chunk_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/chunk_shader.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_TEXTURE_INDEX.at_shader_location(1),
            CUSTOM_UV.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}