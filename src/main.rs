use bevy::{prelude::*, pbr::wireframe::{WireframePlugin, WireframeConfig}, render::render_resource::{SamplerDescriptor, AddressMode, FilterMode}};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin, MovementSettings};

use  prelude::*;

mod chunks;
mod prelude;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                watch_for_changes: true,
                ..Default::default()
            })
            .set(ImagePlugin {
                default_sampler: SamplerDescriptor {
                    address_mode_u: AddressMode::Repeat,
                    address_mode_v: AddressMode::Repeat,
                    address_mode_w: AddressMode::Repeat,
                    mag_filter: FilterMode::Nearest,
                    min_filter: FilterMode::Nearest,
                    ..Default::default()
                },
            })
        )
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .add_plugin(WireframePlugin)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(NoCameraPlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00012, // default: 0.00012
            speed: 40.0,          // default: 12.0
        })
        .init_resource::<PlayerLastPos>()
        .init_resource::<LoadedChunks>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(load_chunk_texture)
        .add_startup_system(setup)
        .add_system(load_chunk_system)
        .run();
}

fn setup(
    mut wireframe_config: ResMut<WireframeConfig>,
    mut commands: Commands
) {
    wireframe_config.global = false;

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(50.0, 50.0, 50.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    }).insert(FlyCam);
}