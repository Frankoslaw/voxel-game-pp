use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}, pbr::wireframe::{WireframeConfig, WireframePlugin}, ecs::schedule::ShouldRun};
use bevy_inspector_egui::{InspectorPlugin, Inspectable, WorldInspectorPlugin};

#[derive(Inspectable, Default)]
pub struct DebugData {
    fps: f64,
    wireframe: bool,
    pause: bool
}

fn display_fps(
    mut debug_data: ResMut<DebugData>,
    diagnostics: Res<Diagnostics>,
){
    let fps_diags = diagnostics
        .get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average());

    if let Some(fps) = fps_diags {
        debug_data.fps = fps;
    } 
}

fn display_wireframe(
    debug_data: Res<DebugData>,
    mut wireframe_config: ResMut<WireframeConfig>,
){
    wireframe_config.global = debug_data.wireframe;
}

fn toggle_wireframe(
    keys: Res<Input<KeyCode>>, 
    mut debug_data: ResMut<DebugData>,
) {
    if keys.just_pressed(KeyCode::Numpad0)  ||
    (keys.pressed(KeyCode::LAlt) && keys.just_pressed(KeyCode::W)) {
        debug_data.wireframe = !debug_data.wireframe;
    }
}

fn tooggle_pause(
    keys: Res<Input<KeyCode>>, 
    mut debug_data: ResMut<DebugData>,
){
    if keys.just_pressed(KeyCode::Numpad1) ||
        (keys.pressed(KeyCode::LAlt) && keys.just_pressed(KeyCode::P)) {
        debug_data.pause = !debug_data.pause;
    }
}

pub fn run_if_not_paused(
    debug_data: Res<DebugData>,
) -> ShouldRun
{
    if !debug_data.pause {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(InspectorPlugin::<DebugData>::new())
            .add_plugin(WireframePlugin)
            .add_plugin(WorldInspectorPlugin::new())
            .add_system(display_fps)
            .add_system(display_wireframe)
            .add_system(toggle_wireframe)
            .add_system(tooggle_pause);
    }
}