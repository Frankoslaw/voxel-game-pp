use bevy::prelude::*;

use crate::{user::player::*, utils::debug::run_if_not_paused};

use super::{voxel_constants::{CHUNK_SIZE, FACE_CHECKS, RENDER_DISTANCE}, chunk_mesh_builder::ChunkMeshBuilder, world::{MyWorldState, BlockType}};

use noise::{NoiseFn, Perlin, Seedable};

#[derive(Component, Default, Clone, Copy)]
pub struct Chunk{
    pub need_update: bool,
    pub blocks: [[[i32; CHUNK_SIZE[2]]; CHUNK_SIZE[1]]; CHUNK_SIZE[0]],
    pub local_pos: IVec3
}

impl Chunk {
    fn new(local_pos: IVec3) -> Self {
        let mut blocks = [[[0; CHUNK_SIZE[2]]; CHUNK_SIZE[1]]; CHUNK_SIZE[0]];
        let perlin = Perlin::new(1);

        for x in 0..CHUNK_SIZE[0] {
            for z in 0..CHUNK_SIZE[2] {
                let real_x = local_pos.x * CHUNK_SIZE[0] as i32 + x as i32;
                let real_z = local_pos.z * CHUNK_SIZE[2] as i32 + z as i32;

                let height = ((perlin.get([
                    real_x as f64 / 100.0, 
                    real_z as f64 / 100.0
                ]) + 1.) * 30.0) as i32;

                for y in 0..CHUNK_SIZE[1] {

                    if (local_pos.y * CHUNK_SIZE[1] as i32 + y as i32) < height{
                        blocks[x][y][z] = 1;
                    }
                }
            }
        }

        Chunk {
            need_update: true,
            blocks,
            local_pos
        }
    }

    fn check_voxel(&self, mesh_builder: &mut ChunkMeshBuilder, pos: [u32; 3], block_types: &Vec<BlockType>){
        if !block_types[self.blocks[pos[0] as usize][pos[1] as usize][pos[2] as usize] as usize].is_solid {
            return;
        }
        
        for i in 0..6{
            let check_x = (pos[0] as i32 + FACE_CHECKS[i][0]).max(0).min(CHUNK_SIZE[0] as i32 - 1) as u32;
            let check_y = (pos[1] as i32 + FACE_CHECKS[i][1]).max(0).min(CHUNK_SIZE[1] as i32 - 1) as u32;
            let check_z = (pos[2] as i32 + FACE_CHECKS[i][2]).max(0).min(CHUNK_SIZE[2] as i32 - 1) as u32;
    
            if (check_x != pos[0] || check_y != pos[1] || check_z != pos[2]) &&
                block_types[self.blocks[check_x as usize][check_y as usize][check_z as usize] as usize].is_solid{
                continue;
            }
    
            mesh_builder.add_face(pos, i as u8);
        }
    }
    
    fn build_mesh(&self, mesh_builder: &mut ChunkMeshBuilder, block_types: &Vec<BlockType>){
        for x in 0..CHUNK_SIZE[0] {
            for y in 0..CHUNK_SIZE[1] {
                for z in 0..CHUNK_SIZE[2] {
                    let pos = [x as u32, y as u32, z as u32];
    
                    self.check_voxel(mesh_builder, pos, block_types);
                }
            }
        }
    }
}

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_not_paused)
                    .with_run_criteria(run_if_player_pos_changed)
                    .with_system(chunk_loader)
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(run_if_not_paused)
                    .with_system(update_chunk)
            );
    }
}

fn update_chunk(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_query: Query<(Entity, &mut Chunk)>,
    game_world: Res<MyWorldState>
){
    let block_types = &game_world.block_types;

    for (entity, mut chunk) in chunk_query.iter_mut() {
        if !chunk.need_update {
            continue;
        }

        let mut mesh_builder = ChunkMeshBuilder::new();
        chunk.build_mesh(&mut mesh_builder, block_types);
        chunk.need_update = false;

        commands
            .entity(entity)
            .despawn();

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(mesh_builder.build()),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform {
                    translation: Vec3 { 
                        x: (chunk.local_pos.x * CHUNK_SIZE[0] as i32) as f32, 
                        y: (chunk.local_pos.y * CHUNK_SIZE[1] as i32) as f32, 
                        z: (chunk.local_pos.z * CHUNK_SIZE[2] as i32) as f32
                    },
                    ..Default::default()
                },
                ..Default::default()
            }).insert(*chunk);
    }
}

fn chunk_loader(
    mut commands: Commands,
    player_query: Query<&Player>,
    chunk_query: Query<(Entity, &Chunk)>,
){
    let mut chunk_to_load = [[[true; (RENDER_DISTANCE*2+1) as usize]; (RENDER_DISTANCE*2+1) as usize]; (RENDER_DISTANCE*2+1) as usize];

    if let Ok(player) = player_query.get_single() {
        for (chunk_entity, chunk) in chunk_query.iter(){
            if  ((chunk.local_pos.x - player.local_pos.x).abs() > RENDER_DISTANCE) || 
                ((chunk.local_pos.y - player.local_pos.y).abs() > RENDER_DISTANCE) ||
                ((chunk.local_pos.z - player.local_pos.z).abs() > RENDER_DISTANCE) {

                commands
                    .entity(chunk_entity)
                    .despawn();

                continue;
            }

            chunk_to_load
                [(RENDER_DISTANCE + chunk.local_pos.x - player.local_pos.x) as usize]
                [(RENDER_DISTANCE + chunk.local_pos.y - player.local_pos.y) as usize]
                [(RENDER_DISTANCE + chunk.local_pos.z - player.local_pos.z) as usize] = false;
        }

        for x in 0..(RENDER_DISTANCE*2+1){
            for y in 0..(RENDER_DISTANCE*2+1){
                for z in 0..(RENDER_DISTANCE*2+1){
                    if !chunk_to_load[x as usize][y as usize][z as usize]{
                        continue;
                    }

                    let chunk = Chunk::new(IVec3 { 
                        x: x + player.local_pos.x - RENDER_DISTANCE, 
                        y: y + player.local_pos.y - RENDER_DISTANCE, 
                        z: z + player.local_pos.z - RENDER_DISTANCE
                    });
    
                    commands
                        .spawn()
                        .insert(chunk);
                }
            }
        }
    }
}