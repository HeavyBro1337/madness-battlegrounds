use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy_rapier3d::control::KinematicCharacterController;

use super::ai::AiUnit;

pub fn move_agents(mut q_units: Query<(&mut KinematicCharacterController, &mut AiUnit, &Transform)>, time: Res<Time>) {
    for (mut cc, mut agent, transform) in q_units.iter_mut() {
        if agent.path.len() == 0 {
            cc.translation = None;
            continue;
        }
        let mut p = agent.path[0];

        p.y = -8.0;

        let dir = ( p - transform.translation).normalize() 
            * agent.speed
            * time.delta_seconds();
        cc.translation = Some(dir);
        if transform.translation.distance(p) <= agent.next_waypoint_distance {
            agent.path.remove(0);
        }
    }
}
