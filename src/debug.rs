use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};

use crate::DebugData;


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

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_system(display_fps);
    }
}