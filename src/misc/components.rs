use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Health {
    pub max_health: f32,
    pub value: f32,
}

impl Default for Health {
    fn default() -> Self {
        return Health {
            max_health: 100.0,
            value: 100.0,
        };
    }
}

#[derive(Debug)]
pub struct DamageDealtEvent {
    pub entity: Entity,
    pub silent: bool,
    pub damage: f32,
}

#[derive(Debug)]
pub struct DeathEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct DamageOverTime {
    pub damage: f32,
    pub silent: bool,
    pub delay: Timer,
}

impl Default for DamageOverTime {
    fn default() -> Self {
        return DamageOverTime {
            damage: 1.0,
            silent: true,
            delay: Timer::from_seconds(0.1, false),
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
