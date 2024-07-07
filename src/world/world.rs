use bevy::{color::palettes, gltf::GltfMesh, prelude::*};
use vleue_navigator::{NavMesh};
use bevy::gltf;

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut navmesh_material: StandardMaterial = Color::Srgba(palettes::css::LIGHT_CYAN).into();
    navmesh_material.unlit = true;
    let gltf_handle = asset_server.load("maps/city_ace.glb");
    let d_gltf = gltfs.get(&gltf_handle).unwrap();
    let scene_handle = asset_server.load("maps/city_ace.glb#Scene0");
    let g_mesh = gltf_meshes.get(&d_gltf.named_meshes["asdwasd"]).unwrap();
    let mesh = meshes.get(&g_mesh.primitives[0].mesh).unwrap();

    let navmesh = NavMesh::from_bevy_mesh(&mesh);

    commands.spawn(
        SceneBundle {
            scene: scene_handle,
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: -9.0,
                z: 0.0,
            }),
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn(PbrBundle {
            mesh: meshes.add(navmesh.to_wireframe_mesh()),
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            material: materials.add(navmesh_material),
            ..default()
        },);
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
