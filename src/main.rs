use bevy::{prelude::*, pbr::wireframe::{WireframeConfig, WireframePlugin}};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, Inspectable, WorldInspectorPlugin};

use chunk::Chunk;
use debug::DebugPlugin;
use fly_cam::FlyCamPlugin;
use world::WorldPlugin;

mod fly_cam;
mod debug;
mod chunk_mesh_builder;
mod chunk;
mod world;
mod voxel_constants;

#[derive(Inspectable, Default)]
struct DebugData {
    fps: f64,
}

#[derive(AssetCollection)]
struct GameAssets {
    #[asset(path = "ReferenceTexture.png")]
    reference_texture: Handle<Image>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum MyStates {
    AssetLoading,
    Next,
}

fn main() {
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
        .add_loading_state(
            LoadingState::new(MyStates::AssetLoading)
                .continue_to_state(MyStates::Next)
                .with_collection::<GameAssets>(),
        )
        .add_plugin(InspectorPlugin::<DebugData>::new())
        //.add_plugin(WorldInspectorPlugin::new())
        .add_state(MyStates::AssetLoading)
        .add_plugin(FlyCamPlugin)
        .add_plugin(WireframePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(WorldPlugin)
        .add_system_set(SystemSet::on_enter(MyStates::Next).with_system(setup))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //assets: Res<GameAssets>,
    mut wireframe_config: ResMut<WireframeConfig>,
) {
    wireframe_config.global = true;

    // let test_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(assets.reference_texture.clone()),
    //     unlit: false,
    //     ..Default::default()
    // });

    let test_material = materials.add(Color::NONE.into());

    let mut spawn_chunk = |x_offset, y_offset, z_offset| {
        commands.spawn_bundle(PbrBundle {
            mesh: meshes.add(Chunk::new().build_mesh()),
            material: test_material.clone(),
            transform: Transform::from_xyz(x_offset, y_offset, z_offset),
            ..Default::default()
        });
    };

    spawn_chunk(0.,0.,0.);
}