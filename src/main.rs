use bevy::prelude::*;
use user::player::PlayerPlugin;
use utils::{fly_cam::FlyCamPlugin, debug::DebugPlugin};
use world::{world::WorldPlugin, chunk::ChunkPlugin};

mod utils;
mod world;
mod user;

fn main(){
  App::new()
    .insert_resource(ClearColor(Color::rgb_u8(51, 153, 255)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .insert_resource(WindowDescriptor {
            title: "Voxel game WIP!".to_string(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        })
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugPlugin)
    .add_plugin(FlyCamPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(WorldPlugin)
    .add_plugin(ChunkPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(){
}