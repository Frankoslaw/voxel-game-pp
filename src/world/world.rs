use bevy::prelude::*;

use super::voxel_constants::RENDER_DISTANCE;


pub struct MyWorldState {
    pub block_types: Vec<BlockType>
}

impl Default for MyWorldState {
    fn default() -> Self {

        MyWorldState { 
            block_types: vec![]
        }
    }
}

pub struct BlockType {
    pub materials: [u8; 6],
    pub is_solid: bool,
    pub name: String
}

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App){
        app
            .insert_resource(MyWorldState {
                block_types: vec![
                    BlockType {
                        materials: [0, 0, 0, 0, 0, 0],
                        is_solid: false,
                        name: String::from("Air")
                    },
                    BlockType {
                        materials: [0, 0, 0, 0, 0, 0],
                        is_solid: true,
                        name: String::from("Stone")
                    }
                ],
                ..Default::default()
            });
    }
}