use crate::misc::MovableComponent;
use bevy::prelude::*;

pub fn move_system(
    time: Res<Time>,
    mut query: Query<(&MovableComponent, &mut Transform), With<MovableComponent>>,
) {
    for (mov, mut e_ray) in query.iter_mut() {
        e_ray.translation += mov.heading * mov.speed * (time.delta_seconds() * 32.0);
    }
}
