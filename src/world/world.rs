use bevy::prelude::*;
use bevy_gltf_collider::get_scene_colliders;

pub fn spawn_map(
    mut commands: Commands,
    mut scenes: ResMut<Assets<Scene>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let scene_handle = asset_server.load("maps/city_ace.glb#Scene0");
    let colliders = get_scene_colliders(
        &mut meshes,
        &mut scenes.get_mut(&scene_handle.clone()).unwrap().world,
    )
    .expect("mesh colliders: okay, clearly something is fucked.");
    commands
        .spawn(SceneBundle {
            scene: scene_handle,
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: -9.0,
                z: 0.0,
            }),
            ..default()
        })
        .with_children(|parent| {
            for (collider, transform) in colliders.iter() {
                parent.spawn((
                    collider.clone(),
                    TransformBundle::from_transform(*transform),
                ));
            }
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
