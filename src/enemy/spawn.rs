use crate::{
    enemy::components::EnemyComponent,
    misc::components::{Health, HitboxComponent, MovableComponent},
};
use bevy::prelude::*;

pub fn spawn_enemy(commands: &mut Commands, spawn: Vec3) {
    let size = 20.0;
    commands
        .spawn()
        .insert(EnemyComponent)
        .insert(Health {
            value: 100.0,
            ..Default::default()
        })
        .insert(HitboxComponent {
            scale: Vec2::new(size, size),
        })
        .insert(MovableComponent {
            speed: 1.0,
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: spawn,
                scale: Vec3::new(size, size, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        });
}
