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

#[derive(Component)]
pub struct MainCamera;

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

#[derive(Component)]
pub struct HitboxComponent {
    pub scale: Vec2,
}

#[derive(Component)]
pub struct MovableComponent {
    pub heading: Vec3, // TODO: use Vec2
    pub speed: f32,
}

impl Default for MovableComponent {
    fn default() -> Self {
        return MovableComponent {
            heading: Vec3::new(0.0, 0.0, 0.0),
            speed: 0.0,
        };
    }
}
