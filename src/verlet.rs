use crate::{Cable, CableNode, Velocity};
use bevy::prelude::*;

const DELTA: f32 = 1.0 / 60.0;
const DELTA_2: f32 = DELTA * DELTA;

pub fn simulate_verlet_system(
    mut node_query: Query<(&mut Transform, &mut Velocity, &mut CableNode), With<Cable>>,
) {
    for (mut transform, velocity, mut cable_node) in node_query.iter_mut() {
        let curr_position = transform.translation;
        transform.translation +=
            (curr_position - cable_node.previous_position) + DELTA_2 * velocity.translation;
        cable_node.previous_position = curr_position;
    }
}
