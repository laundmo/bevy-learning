use crate::{enemy::EnemyComponent, misc::MovableComponent, player::PlayerComponent};
use bevy::prelude::*;

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
