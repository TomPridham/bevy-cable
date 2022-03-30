//1 	function relaxConstraint(particle1, particle2, desiredDistance)
//2 	direction ← normalize(particle2.position − particle1.position)
//3 	Δ𝑑 ← distance(particle1, particle2) − desiredDistance
//4 	particle1.position.add(Δ𝑑direction / 2)
//5 	particle2.position.subtract(Δ𝑑direction / 2)
//(Figure 1.12) A function that projects the positions of two particles on the given distance constraint.

//Now, the Jakobsen method is just doing this repeatedly a number of times:
//1 	function jakobsen(constraints, n)
//2 	repeat 𝑛 times
//3 	for all constraints {particle1, particle2, desiredDistance}
//4 	relaxConstraint(particle1, particle2, desiredDistance)
use crate::{Cable, CableNode, Constraint, Velocity};
use bevy::prelude::*;

pub fn relax_constraint(node_1: &Transform, node_2: &Transform, desired_distance: f32) -> Vec3 {
    let direction = (node_2.translation - node_1.translation).normalize();
    let delta_distance = node_1.translation.distance(node_2.translation) - desired_distance;
    delta_distance * direction / 2.0
}

pub fn jakobsen_system(
    constraint_query: Query<&Constraint>,
    mut node_query: Query<(&mut Transform, &mut Velocity), With<Cable>>,
) {
    for _ in 0..5 {
        for Constraint {
            node_1,
            node_2,
            desired_distance,
        } in constraint_query.iter()
        {
            let (t1, _) = if let Ok(c) = node_query.get(*node_1) {
                c
            } else {
                continue;
            };
            let (t2, _) = if let Ok(c) = node_query.get(*node_2) {
                c
            } else {
                continue;
            };
            let d1 = relax_constraint(&t1, &t2, *desired_distance);
            let (mut t1, mut v1) = node_query.get_mut(*node_1).unwrap();
            t1.translation += d1;
            let (mut t2, mut v2) = node_query.get_mut(*node_2).unwrap();
            t2.translation -= d1;
        }
    }
}
