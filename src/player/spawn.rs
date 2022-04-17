use bevy::prelude::*;

use crate::misc::components::{Health, MovableComponent};

use super::PlayerComponent;

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(PlayerComponent {
            base_damage: 100.0,
            ..default()
        })
        .insert(Health {
            value: 1000.0,
            ..default()
        })
        .insert(MovableComponent { ..default() })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(20.0, 20.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        });
}
