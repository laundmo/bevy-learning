use crate::misc::components::DamageDealtEvent;
use crate::{
    enemy::components::EnemyComponent, enemy::spawn_enemy, misc::components::MovableComponent,
    player::PlayerComponent,
};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;

use super::resources::EnemySpawnTimer;

const SPAWN_RANGE: f32 = 500.0;

const VS: &[(f32, f32)] = &[
    (SPAWN_RANGE, 0.0),
    (SPAWN_RANGE, SPAWN_RANGE),
    (0.0, SPAWN_RANGE),
    (-SPAWN_RANGE, SPAWN_RANGE),
    (-SPAWN_RANGE, 0.0),
    (-SPAWN_RANGE, -SPAWN_RANGE),
    (0.0, -SPAWN_RANGE),
    (SPAWN_RANGE, -SPAWN_RANGE),
];

pub fn spawn_enemies_system(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    mut query: Query<&mut Transform, With<PlayerComponent>>,
) {
    if enemy_timer.delay.tick(time.delta()).finished() {
        let p_ray = query.single_mut();
        let mut rng = rand::thread_rng();
        let side = VS.choose(&mut rng).unwrap();

        let spawn = Vec3::new(
            p_ray.translation.x + side.0 + rng.gen_range(-50.0..50.0),
            p_ray.translation.y + side.1 + rng.gen_range(-50.0..50.0),
            0.0,
        );
        spawn_enemy(&mut commands, spawn);
        enemy_timer.delay.reset();
    }
}

pub fn enemy_targeting_system(
    mut player_query: Query<&mut Transform, (With<PlayerComponent>, Without<EnemyComponent>)>,
    mut query: Query<(&mut MovableComponent, &mut Transform), With<EnemyComponent>>,
) {
    let p_ray = player_query.single_mut();

    for (mut mov, e_ray) in query.iter_mut() {
        let towards = (p_ray.translation - e_ray.translation).normalize();
        mov.heading = towards;
    }
}

// pub fn enemy_damage_indicator(
//     mut reader: EventReader<DamageDealtEvent>,
//     mut query: Query<&mut Sprite, With<EnemyComponent>>,
// ) {
//     for evt in reader.iter() {
//         if let Ok(sprite) = query.get(evt.entity) {
//             sprite.color
//         }
//     }
// }
