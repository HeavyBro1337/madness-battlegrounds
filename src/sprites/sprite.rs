use core::hash;

use bevy::prelude::*;
use bevy::utils::hashbrown::HashSet;
use bevy_rapier3d::dynamics::RigidBody;
use bevy_rapier3d::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dComponent, Sprite3dParams};

use rand::thread_rng;
use rand::Rng;

use crate::{ai::ai::AiUnit, loading::loading::ImageLayouts};

use super::animation::{AnimationTimer, CurrentAnimationFrameCount};

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
    image_layouts: Res<ImageLayouts>,
) {
    let texture_atlas = TextureAtlas {
        layout: image_layouts
            .0
            .get(&"sprites/unit_idle.png".to_string())
            .unwrap()
            .clone(),
        index: 1,
    };

    let mut hashset = HashSet::new();

    for _ in 0..750 {
        let point: IVec2 = IVec2::new(rand::thread_rng().gen_range(-9..9), rand::thread_rng().gen_range(-9..9));

        if hashset.contains(&point) {
            continue;
        }
        hashset.insert(point);
        commands
            .spawn((
                Sprite3d {
                    transform: Transform::from_translation(Vec3 {
                        x: point.x as f32,
                        y: -7.5,
                        z: point.y as f32,
                    }),
                    image: asset_server.load("sprites/unit_idle.png"),
                    pixels_per_metre: 32.,
                    pivot: Some(Vec2::new(0.5, 0.0)),
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    ..default()
                }
                .bundle_with_atlas(&mut sprite_params, texture_atlas.clone()),
                AiUnit::new(0.25, 5.0),
                KinematicCharacterController::default(),
                RigidBody::KinematicPositionBased,
                Collider::ball(0.25)
            ))
            .insert((
                AnimationTimer::new(Timer::from_seconds(0.1, TimerMode::Repeating)),
                CurrentAnimationFrameCount(6),
            ));
    }
}
