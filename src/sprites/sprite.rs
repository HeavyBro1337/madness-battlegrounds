use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dBundle, Sprite3dComponent, Sprite3dParams};

use crate::loading::loading::ImageLayouts;

pub fn rotate_sprites_to_camera(
    mut q_sprites: Query<&mut Transform, (With<Sprite3dComponent>, Without<Camera3d>)>,
    q_cam_transform: Query<&Transform, (With<Camera>, Without<Sprite3dComponent>)>,
) {
    let commander_cam = q_cam_transform.single();

    for mut transform in q_sprites.iter_mut() {
        let v = commander_cam.forward().as_dvec3().as_vec3();

        transform.look_to(v, Vec3::Y);
    }
}

pub fn spawn_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprite_params: Sprite3dParams,
    mut image_layouts: ImageLayouts
) {
    let texture_atlas = TextureAtlas {
        layout: image_layouts.0.get(&"sprites/unit_idle.png".to_string()).unwrap().clone(),
        index: 1,
    };
    
    commands.spawn(
        Sprite3d {
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: -30.0,
                z: 0.0,
            }),
            image: asset_server.load("sprites/unit_idle.png"),
            pixels_per_metre: 32.,
            pivot: None,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            ..default()
        }
        .bundle_with_atlas(&mut sprite_params, texture_atlas),
    );
}
