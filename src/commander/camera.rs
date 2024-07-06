use bevy::{
    prelude::*,
    prelude::{default, Camera3dBundle, Commands, Component},
    render::camera::OrthographicProjection,
};

#[derive(Component)]
pub struct CommanderCamera;

const COMMANDER_POSITION: Vec3 = Vec3 {
    x: 0.0,
    y: 10.0,
    z: 0.0,
};

pub fn setup_commander_camera(mut commands: Commands) {
    use bevy::render::camera::Projection::Orthographic;

    let projection = OrthographicProjection {
        scaling_mode: bevy::render::camera::ScalingMode::FixedHorizontal(32.0),
        ..default()
    };

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection::default()),
            transform: Transform::from_translation(COMMANDER_POSITION)
                .looking_at(Vec3::X / 2.0, Vec3::Y),
            ..default()
        },
        CommanderCamera,
    ));
}
