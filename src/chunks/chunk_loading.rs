use bevy::utils::HashMap;
use bevy_flycam::FlyCam;

use crate::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerLastPos{
    pos: IVec3
}

impl PlayerLastPos{
    pub fn from_world_pos(pos: Vec3) -> IVec3 {
        IVec3::new(
            (pos.x / CHUNK_SIZE as f32).floor() as i32,
            (pos.y / CHUNK_SIZE as f32).floor() as i32,
            (pos.z / CHUNK_SIZE as f32).floor() as i32,
        )
    }
}

#[derive(Default, Resource)]
pub struct LoadedChunks{
    ent: HashMap<IVec3, Entity>
}

pub fn load_chunk_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<FlyCam>>,
    mut player_last_pos: ResMut<PlayerLastPos>,
    mut loaded_chunks: ResMut<LoadedChunks>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_material: ResMut<Assets<ChunkMaterial>>,
    texture: Res<ChunkTexture>,
){
    if let Ok(player_transform) = player_query.get_single() {
        let player_local_pos = PlayerLastPos::from_world_pos(player_transform.translation);

        if player_last_pos.pos != player_local_pos{
            player_last_pos.pos = player_local_pos;


            for x in -RENDER_DISTANCE - 1..RENDER_DISTANCE + 2 {
                for y in -RENDER_DISTANCE - 1..RENDER_DISTANCE + 2 {
                    for z in -RENDER_DISTANCE - 1..RENDER_DISTANCE + 2 {
                        let chunk_pos = IVec3::new(
                            player_local_pos.x + x, 
                            player_local_pos.y + y, 
                            player_local_pos.z + z
                        );

                        if  x == (-RENDER_DISTANCE - 1) ||  x == (RENDER_DISTANCE + 1) ||
                            y == (-RENDER_DISTANCE - 1) ||  y == (RENDER_DISTANCE + 1) ||
                            z == (-RENDER_DISTANCE - 1) ||  z == (RENDER_DISTANCE + 1) {

                            // Despawn chunks outside of render distance
                            if loaded_chunks.ent.contains_key(&chunk_pos) {
                                commands.entity(*loaded_chunks.ent.get(&chunk_pos).unwrap()).despawn();
                                loaded_chunks.ent.remove(&chunk_pos);
                            }
                            continue;
                        }

                        if loaded_chunks.ent.contains_key(&chunk_pos){
                            continue;
                        }

                        // Spawn new chunks
                        let chunk = Chunk::chunk_from_pos(chunk_pos);
                        let mesh = chunk_mesh_generation(&chunk);

                        let chunk_ent = commands.spawn(MaterialMeshBundle {
                            material: chunk_material.add(ChunkMaterial {
                                textures: texture.0.clone(),
                            }).clone(),
                            transform: Transform {
                                translation: chunk.get_world_cords(),
                                ..default()
                            },
                            mesh: meshes.add(mesh.clone()),
                            ..default()
                        })
                            .id();

                        loaded_chunks.ent.insert(chunk_pos, chunk_ent);
                    }
                }
            }
        }
    }
}