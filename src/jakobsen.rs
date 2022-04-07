use crate::{Cable, CableNode, Constraint};
use bevy::prelude::*;

pub fn relax_constraint(node_1: &Transform, node_2: &Transform, desired_distance: f32) -> Vec3 {
    let direction = (node_2.translation - node_1.translation).normalize();
    let delta_distance = node_1.translation.distance(node_2.translation) - desired_distance;
    delta_distance * direction / 2.0
}

pub fn jakobsen_system(
    constraint_query: Query<&Constraint>,
    mut node_query: Query<(&mut Transform, &CableNode), With<Cable>>,
) {
    for _ in 0..5 {
        for Constraint {
            node_1,
            node_2,
            desired_distance,
        } in constraint_query.iter()
        {
            let (t1, cable_node_1) = if let Ok(nq) = node_query.get(*node_1) {
                nq
            } else {
                continue;
            };
            let (t2, cable_node_2) = if let Ok(nq) = node_query.get(*node_2) {
                nq
            } else {
                continue;
            };

            let cable_node_1_fixed = cable_node_1.fixed;
            let cable_node_2_fixed = cable_node_2.fixed;

            let d1 = relax_constraint(t1, t2, *desired_distance);

            let (mut t1, _) = node_query.get_mut(*node_1).unwrap();
            if !cable_node_1_fixed {
                let distance_factor = if cable_node_2_fixed { 2.0 } else { 1.0 };
                t1.translation += d1 * distance_factor;
            }
            let (mut t2, _) = node_query.get_mut(*node_2).unwrap();
            if !cable_node_2_fixed {
                let distance_factor = if cable_node_1_fixed { 2.0 } else { 1.0 };
                t2.translation -= d1 * distance_factor;
            }
        }
    }
}
