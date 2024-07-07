use std::f32::consts::FRAC_PI_2;

use bevy::{
    color::palettes,
    math::vec2,
    pbr::NotShadowCaster,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use vleue_navigator::{
    prelude::{NavMeshBundle, NavMeshSettings, NavMeshStatus, NavMeshUpdateMode},
    NavMesh, Path, Triangulation,
};

const MESH_WIDTH: u32 = 150;
const MESH_HEIGHT: u32 = 100;

pub fn setup_navmesh(mut commands: Commands) {
    commands.spawn(NavMeshBundle {
        settings: NavMeshSettings {
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&vec![
                vec2(0.0, 0.0),
                vec2(MESH_WIDTH as f32, 0.0),
                vec2(MESH_WIDTH as f32, MESH_HEIGHT as f32),
                vec2(0.0, MESH_HEIGHT as f32),
            ]),
            simplify: 0.001,
            merge_steps: 0,

            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
        // Mark it for update as soon as obstacles are changed.
        // Other modes can be debounced or manually triggered.
        update_mode: NavMeshUpdateMode::Direct,
        ..default()
    });
}

pub fn display_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    navmeshes: Res<Assets<NavMesh>>,
    mut current_mesh_entity: Local<Option<Entity>>,
    navmesh: Query<(&Handle<NavMesh>, Ref<NavMeshStatus>)>,
) {
    let (navmesh_handle, status) = navmesh.single();
    if !status.is_changed() {
        return;
    }

    let Some(navmesh) = navmeshes.get(navmesh_handle) else {
        return;
    };
    if let Some(entity) = *current_mesh_entity {
        commands.entity(entity).despawn_recursive();
    }

    *current_mesh_entity = Some(
        commands
            .spawn(PbrBundle {
                mesh: meshes
                    .add(Plane3d::new(
                        Vec3::Y,
                        Vec2::new(MESH_WIDTH as f32 / 2.0, MESH_HEIGHT as f32 / 2.0),
                    ))
                    .into(),
                transform: Transform::from_translation(Vec3::new(
                    (MESH_WIDTH as f32) / 2.0,
                    0.0,
                    MESH_HEIGHT as f32 / 2.0,
                )),
                material: materials.add(StandardMaterial::from(Color::Srgba(
                    palettes::tailwind::BLUE_800,
                ))),
                ..default()
            })
            .with_children(|main_mesh| {
                main_mesh.spawn((
                    PbrBundle {
                        mesh: meshes.add(navmesh.to_wireframe_mesh()).into(),
                        material: materials.add(StandardMaterial::from(Color::Srgba(
                            palettes::tailwind::BLUE_400,
                        ))),
                        transform: Transform::from_translation(Vec3::new(
                            -(MESH_WIDTH as f32) / 2.0,
                            0.1,
                            -(MESH_HEIGHT as f32) / 2.0,
                        )),
                        ..default()
                    },
                    NotShadowCaster,
                ));
            })
            .id(),
    );
}
