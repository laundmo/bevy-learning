use bevy::prelude::*;

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
