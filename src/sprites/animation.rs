use bevy::prelude::*;
use bevy::{
    prelude::{Component, Query},
    sprite::TextureAtlas,
    time::Timer,
};

#[derive(Default, Component)]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn new(timer: Timer) -> Self {
        AnimationTimer(timer)
    }
}

#[derive(Default, Component)]
pub struct CurrentAnimationFrameCount(pub usize);

pub fn animate_sprite(
    time: Res<Time>,
    mut q_sprite: Query<(
        &mut TextureAtlas,
        &mut AnimationTimer,
        &CurrentAnimationFrameCount,
    )>,
) {
    for (mut atlas, mut timer, frame_count) in q_sprite.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            atlas.index = (atlas.index + 1) % frame_count.0;
        }
    }
}
