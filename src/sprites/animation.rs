use bevy::{prelude::Component, time::Timer};

#[derive(Default, Component)]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn new(timer: Timer) -> Self {
        AnimationTimer(timer)
    }
}

#[derive(Default, Component)]
pub struct CurrentAnimationFrameCount(pub usize);
