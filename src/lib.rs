mod jakobsen;
mod verlet;

pub use crate::jakobsen::jakobsen_system;
pub use crate::verlet::simulate_verlet_system;

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Velocity {
    pub translation: Vec3,
}
#[derive(Component)]
pub struct CableNode {
    pub previous_position: Vec3,
    pub fixed: bool,
}

#[derive(Component)]
pub struct CableHead;

#[derive(Component)]
pub struct Cable;

#[derive(Component)]
pub struct Constraint {
    pub node_1: Entity,
    pub node_2: Entity,
    pub desired_distance: f32,
}

const VERLET: &str = "verlet";

pub struct CablePlugin;

impl Plugin for CablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(simulate_verlet_system.label(VERLET))
            .add_system(jakobsen_system.after(VERLET));
    }
}
