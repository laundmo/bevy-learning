use bevy::prelude::*;

#[derive(Component)]
pub struct DamageOverTime {
    pub damage: f32,
    pub delay: Timer,
}

impl Default for DamageOverTime {
    fn default() -> Self {
        return DamageOverTime {
            damage: 1.0,
            delay: Timer::from_seconds(0.05, false),
        };
    }
}
