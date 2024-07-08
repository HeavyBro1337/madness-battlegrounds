use bevy::prelude::*;
use bevy::transform::components::Transform;

use super::ai::AiUnit;

pub fn move_agents(mut q_units: Query<(&mut Transform, &mut AiUnit)>, time: Res<Time>) {
    for (mut transform, mut agent) in q_units.iter_mut() {
        if agent.path.len() == 0 {
            continue;
        }

        let dir = (agent.path[0] - transform.translation).normalize()
            * agent.speed
            * time.delta_seconds();
        transform.translation += dir;
        if transform.translation.distance(agent.path[0]) <= agent.next_waypoint_distance {
            agent.path.remove(0);
        }
    }
}
