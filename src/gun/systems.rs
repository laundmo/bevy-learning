use crate::{bullet::spawn_bullet, misc::resources::MouseCoords};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

use super::components::Gun;

pub fn fire(
    mouse: Res<MouseCoords>,
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut owner: Query<(&mut Gun, &Transform), With<Gun>>,
) {
    let (mut gun, owner_ray) = owner.single_mut();
    let left_mouse = mouse_input.pressed(MouseButton::Left);

    if gun.fire_timer.tick(time.delta()).finished()
        && (left_mouse || keyboard_input.pressed(KeyCode::F))
    {
        spawn_bullet(
            &mut commands,
            owner_ray.translation.xy(),
            gun.damage,
            (mouse.pos - owner_ray.translation.xy()).normalize(),
        );
        gun.fire_timer.reset();
    }
}
