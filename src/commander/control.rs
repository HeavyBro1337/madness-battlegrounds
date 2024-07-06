use bevy::{
    input::ButtonInput,
    prelude::{Event, KeyCode, Query, Res, With},
    render::camera::Camera,
    time::Time,
    transform::components::Transform,
};

use super::camera::CommanderCamera;

pub fn move_camera(
    mut q_commander_transform: Query<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    return;

    const SPEED: f32 = 300.0;

    let mut transform = q_commander_transform.single_mut();

    if keyboard.pressed(KeyCode::KeyD) {
        transform.translation.x += SPEED * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::KeyA) {
        transform.translation.x -= SPEED * time.delta_seconds();
    }

    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation.z += SPEED * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation.z -= SPEED * time.delta_seconds();
    }
}
