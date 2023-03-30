#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

#import bevy_pbr::mesh_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) texture_i: u32,
    @location(2) uvs: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_i: u32,
    @location(1) uvs: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.texture_i = vertex.texture_i;
    out.uvs = vertex.uvs;
    return out;
}

struct FragmentInput {
    @location(0) texture_i: u32,
    @location(1) uvs: vec2<f32>,
};

@fragment
fn fragment(
    input: FragmentInput
) -> @location(0) vec4<f32> {
    let id = i32(input.texture_i);
    let tex_x = f32(id % 32);
    let tex_y = f32(i32(id / 32));
    return textureSample(texture, texture_sampler, vec2<f32>(
        (input.uvs.x % 1. + tex_x) / 32., 
        (input.uvs.y % 1. + tex_y) / 32.
    ));
}