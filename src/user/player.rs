use bevy::{prelude::*, ecs::schedule::ShouldRun};

use crate::world::voxel_constants::CHUNK_SIZE;


#[derive(Component)]
pub struct Player{
    pub local_pos: IVec3,
    pub last_local_pos: IVec3,
}

impl Player {
    pub fn global_to_local_pos(global_pos: Vec3) -> IVec3{
        IVec3 {
            x: (global_pos.x / CHUNK_SIZE[0] as f32).floor() as i32,
            y: (global_pos.y / CHUNK_SIZE[1] as f32).floor() as i32,
            z: (global_pos.z / CHUNK_SIZE[2] as f32).floor() as i32,
        }
    }
}

impl Default for Player{
    fn default() -> Self {
        Player { 
            last_local_pos: IVec3::new(0, 0, 0), 
            local_pos: IVec3::new(0, 0, 0)
        }
    }
}

pub fn run_if_player_pos_changed(
    mut player_query: Query<(&Transform, &mut Player)>
) -> ShouldRun
{
    if let Ok((player_tf, mut player)) = player_query.get_single_mut() {
        player.local_pos = Player::global_to_local_pos(player_tf.translation);

        if Player::global_to_local_pos(player_tf.translation) == player.last_local_pos {
            return ShouldRun::No
        }

        player.last_local_pos = Player::global_to_local_pos(player_tf.translation);

        return ShouldRun::Yes
    }

    return ShouldRun::No
}