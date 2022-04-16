use crate::{
    enemy::EnemyComponent,
    misc::{Health, HitboxComponent, MovableComponent, NameComponent},
};
use bevy::prelude::*;

pub fn spawn_enemy(commands: &mut Commands, spawn: Vec3) {
    commands
        .spawn()
        .insert(EnemyComponent)
        .insert(Health {
            value: 100.0,
            ..Default::default()
        })
        .insert(HitboxComponent {
            scale: Vec2::new(8.0, 8.0),
        })
        .insert(MovableComponent {
            speed: 1.0,
            ..Default::default()
        })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: spawn,
                scale: Vec3::new(8.0, 8.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NameComponent("enemy".to_string()));
}
