use bevy::prelude::*;

#[derive(Component)]
pub struct Health {
    pub value: f32,
}

impl Default for Health {
    fn default() -> Self {
        return Health { value: f32::MAX };
    }
}

pub struct DamageDealtEvent {
    pub entity: Entity,
    pub damage: f32,
}

pub struct DeathEvent {
    pub entity: Entity,
}
