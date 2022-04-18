use bevy::prelude::*;

use crate::{
    gun::components::Gun,
    misc::components::{Health, MovableComponent},
};

use super::PlayerComponent;

pub fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(PlayerComponent)
        .insert(Gun {
            damage: 10.0,
            fire_timer: Timer::from_seconds(0.1, false),
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
