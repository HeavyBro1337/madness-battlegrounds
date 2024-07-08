use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{math::Vec3, prelude::Resource};

use super::camera::CommanderCamera;

#[derive(Resource, Default)]
pub struct Cursor3DCoordinates(pub Vec3);

fn cursor_to_ground_plane(
    mut mouse_coords: ResMut<Cursor3DCoordinates>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<CommanderCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    const GROUND_Y: f32 = -9.0;

    let window = q_window.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let plane = Plane3d::new(Vec3::Y);
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let Some(distance) = ray.intersect_plane(Vec3::Y * GROUND_Y, plane) else {
        return;
    };

    let global_cursor: Vec3 = ray.get_point(distance);

    mouse_coords.0 = global_cursor;
}

pub struct Cursor3dPlugin;

impl Plugin for Cursor3dPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Cursor3DCoordinates::default())
            .add_systems(Update, cursor_to_ground_plane);
    }
}
