use crate::{Cable, Constraint, Velocity};
use bevy::prelude::*;

pub fn relax_constraint(
    node_1: &GlobalTransform,
    node_2: &GlobalTransform,
    desired_distance: f32,
) -> Vec3 {
    let direction = (node_2.translation - node_1.translation).normalize();
    let delta_distance = node_1.translation.distance(node_2.translation) - desired_distance;
    delta_distance * direction / 2.0
}

pub fn jakobsen_system(
    constraint_query: Query<&Constraint>,
    global_query: Query<&GlobalTransform, With<Cable>>,
    mut node_query: Query<(&mut Transform, &mut Velocity), With<Cable>>,
) {
    for _ in 0..5 {
        for Constraint {
            node_1,
            node_2,
            desired_distance,
        } in constraint_query.iter()
        {
            let gt1 = if let Ok(c) = global_query.get(*node_1) {
                c
            } else {
                continue;
            };
            let gt2 = if let Ok(c) = global_query.get(*node_2) {
                c
            } else {
                continue;
            };
            let d1 = relax_constraint(gt1, gt2, *desired_distance);
            let (mut t1, _v1) = node_query.get_mut(*node_1).unwrap();
            t1.translation += d1;
            let (mut t2, _v2) = node_query.get_mut(*node_2).unwrap();
            t2.translation -= d1;
        }
    }
}
