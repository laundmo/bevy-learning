use crate::{
    bullet::BulletComponent,
    misc::{Health, HitboxComponent, MovableComponent, NameComponent},
};
use bevy::prelude::*;

pub fn spawn_bullet(commands: &mut Commands, spawn_: Vec2, damage: f32, heading_: Vec2) {
    let heading = Vec3::new(heading_.x, heading_.y, 0.0);
    let spawn = Vec3::new(spawn_.x, spawn_.y, 0.0);
    commands
        .spawn()
        .insert(BulletComponent { damage })
        .insert(HitboxComponent {
            scale: Vec2::new(8.0, 8.0),
        })
        .insert(Health {
            ..Default::default()
        })
        .insert(MovableComponent {
            speed: 3.0,
            heading,
        })
        .insert_bundle(SpriteBundle {
            transform: Transform {
                translation: spawn,
                scale: Vec3::new(8.0, 8.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NameComponent("enemy".to_string()));
}
