use bevy::prelude::*;
use bevy_cable::*;

const SPRITE_SIZE: f32 = 75.0;
const VERLET: &str = "verlet";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(simulate_verlet_system.label(VERLET))
        .add_system(jakobsen_system.after(VERLET))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let head = commands
        .spawn_bundle((
            Velocity {
                translation: Vec3::ZERO,
            },
            CableHead,
            Cable,
        ))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                color: Color::LIME_GREEN,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();
    commands.entity(head).with_children(|parent| {
        let colors = vec![Color::PINK, Color::RED, Color::SILVER, Color::YELLOW];
        let transform = vec![
            Transform::from_xyz(30.0, 45.0, 0.0),
            Transform::from_xyz(60.0, 85.0, 0.0),
            Transform::from_xyz(90.0, 115.0, 0.0),
            Transform::from_xyz(120.0, 145.0, 0.0),
        ];
        let mut prev_id = head;
        for i in 0..4 {
            let id = parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(SPRITE_SIZE, SPRITE_SIZE)),
                        color: colors[i],
                        ..Default::default()
                    },
                    transform: transform[i],
                    ..Default::default()
                })
                .insert_bundle((
                    CableNode {
                        previous_position: transform[i].translation,
                    },
                    Velocity {
                        translation: Vec3::new(0.0, -9.8, 0.0),
                    },
                    Cable,
                ))
                .id();
            parent.spawn().insert(Constraint {
                node_1: prev_id,
                node_2: id,
                desired_distance: SPRITE_SIZE,
            });
            prev_id = id;
        }
    });
}
