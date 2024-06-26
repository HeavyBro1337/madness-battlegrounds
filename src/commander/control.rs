use bevy::{prelude::{Query, With}, transform::components::Transform};

use super::camera::CommanderCamera;

pub fn move_camera(mut q_commander_transform: Query<&mut Transform, With<CommanderCamera>>) {
    let transform = q_commander_transform.single_mut();

    
}