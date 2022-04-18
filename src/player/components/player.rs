use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent {
    pub base_damage: f32,
    pub fire_timer: Timer,
}

impl Default for PlayerComponent {
    fn default() -> Self {
        return PlayerComponent {
            base_damage: 10.0,
            fire_timer: Timer::from_seconds(0.1, false),
        };
    }
}
