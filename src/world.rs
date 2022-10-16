use bevy::prelude::*;

pub struct World {
    block_types: Vec<BlockType> 
}

pub struct BlockType {
    blockName: String,
    isSolid: bool
} 

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(
        &self, 
        app: &mut App
    ){
        app
            .insert_resource(World {
                block_types: vec![
                    BlockType {
                        blockName: String::from("Dirt"),
                        isSolid: true
                    }
                ]
            });
    }
}