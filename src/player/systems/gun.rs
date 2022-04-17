use crate::{bullet::spawn_bullet, misc::resources::MouseCoords, player::PlayerComponent};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;

pub fn player_fire_gun(
    mouse: Res<MouseCoords>,
    mut commands: Commands,
    time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut PlayerComponent, &mut Transform), With<PlayerComponent>>,
) {
    let (mut player, player_ray) = player_query.single_mut();
    let left_mouse = mouse_input.pressed(MouseButton::Left);

    if player.fire_timer.tick(time.delta()).finished()
        && (left_mouse || keyboard_input.pressed(KeyCode::F))
    {
        spawn_bullet(
            &mut commands,
            player_ray.translation.xy(),
            player.base_damage,
            (mouse.pos - player_ray.translation.xy()).normalize(),
        );
        player.fire_timer.reset();
    }
}
