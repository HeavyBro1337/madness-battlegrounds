use bevy::prelude::*;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_handle = asset_server.load("maps/city_ace.glb#Scene0");

    commands.spawn(SceneBundle {
        scene: scene_handle,
        transform: Transform::from_translation(Vec3 {
            x: 0.0,
            y: -9.0,
            z: 0.0,
        }),
        ..default()
    });
}

pub fn spawn_sun(mut commands: Commands) {
    let bundle = DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            ..default()
        },
        transform: Transform::default().looking_at(-Vec3::Y, Vec3::Y),
        ..default()
    };
    commands.spawn(bundle);
}
