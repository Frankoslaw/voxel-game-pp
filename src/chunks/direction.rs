use std::{
    f32::consts::PI,
    ops::{Index, IndexMut},
};

use bevy::prelude::*;
use block_mesh::ilattice::glam;

#[derive(Clone, Copy)]
pub enum VoxelDirection {
    Front = 0,  // x + 1
    Back = 1,   // x - 1
    Left = 2,   // z + 1
    Right = 3,  // z - 1
    Top = 4,    // y + 1
    Bottom = 5, // y - 1
}

impl<T> Index<VoxelDirection> for [T; 6] {
    type Output = T;

    fn index(&self, index: VoxelDirection) -> &Self::Output {
        &self[index as usize]
    }
}

impl<T> IndexMut<VoxelDirection> for [T; 6] {
    fn index_mut(&mut self, index: VoxelDirection) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
impl VoxelDirection {
    pub fn get_face_rotation(&self) -> Quat {
        match self {
            VoxelDirection::Front => Quat::from_axis_angle(Vec3::Y, PI / 2.0),
            VoxelDirection::Back => Quat::from_axis_angle(Vec3::Y, -PI / 2.0),
            VoxelDirection::Top => Quat::from_axis_angle(Vec3::X, -PI / 2.0),
            VoxelDirection::Bottom => Quat::from_axis_angle(Vec3::X, PI / 2.0),
            VoxelDirection::Left => Quat::from_axis_angle(Vec3::Y, 0.0),
            VoxelDirection::Right => Quat::from_axis_angle(Vec3::Y, PI),
        }
    }

    pub fn opposite(&self) -> VoxelDirection {
        match self {
            VoxelDirection::Front => VoxelDirection::Back,
            VoxelDirection::Back => VoxelDirection::Front,
            VoxelDirection::Left => VoxelDirection::Right,
            VoxelDirection::Right => VoxelDirection::Left,
            VoxelDirection::Top => VoxelDirection::Bottom,
            VoxelDirection::Bottom => VoxelDirection::Top,
        }
    }

    pub fn from_ivec(vec: glam::IVec3) -> VoxelDirection{
        if vec.y > 0{
            VoxelDirection::Top
        }else if vec.y < 0{
            VoxelDirection::Bottom
        }else if vec.x > 0{
            VoxelDirection::Right
        }else if vec.x < 0{
            VoxelDirection::Left
        }else if vec.z > 0{
            VoxelDirection::Front
        }else{
            VoxelDirection::Back
        }
    }
}