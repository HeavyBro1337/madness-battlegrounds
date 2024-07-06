use bevy::{
    prelude::*,
    prelude::{default, Camera3dBundle, Commands, Component},
};

#[derive(Component)]
pub struct CommanderCamera;

const COMMANDER_POSITION: Vec3 = Vec3 {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub fn setup_commander_camera(mut commands: Commands) {
    let projection = PerspectiveProjection { ..default() };

    commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(projection),
            transform: Transform::from_translation(COMMANDER_POSITION)
                .looking_at(-Vec3::Z / 2.0, Vec3::Y),
            ..default()
        },
        CommanderCamera,
    ));
}
