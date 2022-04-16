use bevy::prelude::*;

use crate::{misc::MovableComponent, player::PlayerComponent};

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut MovableComponent, With<PlayerComponent>>,
) {
    let mut mov = query.single_mut();

    let up = keyboard_input.pressed(KeyCode::W);
    let down = keyboard_input.pressed(KeyCode::S);
    let left = keyboard_input.pressed(KeyCode::A);
    let right = keyboard_input.pressed(KeyCode::D);
    let shift = keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);

    let mut head = Vec3::new(0.0, 0.0, 0.0);
    if up {
        head.y += 4.0;
    }
    if down {
        head.y -= 4.0;
    }
    if left {
        head.x -= 4.0;
    }
    if right {
        head.x += 4.0;
    }
    mov.heading = head;
    if shift {
        mov.speed = 3.0;
    } else {
        mov.speed = 1.0;
    }
}
