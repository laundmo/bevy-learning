use bevy::prelude::*;

#[derive(Component)]
pub struct Gun {
    pub damage: f32,
    pub fire_timer: Timer,
}
