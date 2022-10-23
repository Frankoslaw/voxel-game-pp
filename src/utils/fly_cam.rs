use bevy::ecs::event::{ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::{window::{Window, Windows}};

use crate::user::player::Player;


// Mouse data
#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 30.,
        }
    }
}

#[derive(Component)]
pub struct FlyCam;

fn toggle_grab_cursor(
    window: &mut Window
) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn setup_player(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 5.0, 5.0),
            ..Default::default()
        })
        .insert(FlyCam)
        .insert(Player::default());
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    mut query: Query<&mut Transform, With<FlyCam>>
){
    if let Some(window) = windows.get_primary() {
        for mut transform in query.iter_mut(){
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed(){
                if window.cursor_locked() {
                    match key {
                        KeyCode::W => velocity += forward,
                        KeyCode::S => velocity -= forward,
                        KeyCode::A => velocity -= right,
                        KeyCode::D => velocity += right,
                        KeyCode::Space => velocity += Vec3::Y,
                        KeyCode::LShift => velocity -= Vec3::Y,
                        _ => (),
                    }
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * settings.speed;
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                if window.cursor_locked() {
                    let window_scale = window.height().min(window.width());
                    delta_state.pitch -=
                        (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                }

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

pub struct FlyCamPlugin;
impl Plugin for FlyCamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(setup_player)
            .add_startup_system(initial_grab_cursor)
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}