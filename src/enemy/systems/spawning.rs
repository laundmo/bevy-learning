use crate::{enemy::spawn_enemy, player::PlayerComponent};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::Rng;
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
    mut query: Query<&mut Transform, With<PlayerComponent>>,
) {
    let p_ray = query.single_mut();
    let mut rng = rand::thread_rng();
    for _ in 0..1 {
        let side = VS.choose(&mut rng).unwrap();

        let spawn = Vec3::new(
            p_ray.translation.x + side.0 + rng.gen_range(-50.0..50.0),
            p_ray.translation.y + side.1 + rng.gen_range(-50.0..50.0),
            0.0,
        );
        spawn_enemy(&mut commands, spawn);
    }
}