use bevy::prelude::*;

#[derive(Default)]
pub struct MouseCoords {
    pub pos: Vec2,
}

#[derive(Default)]
pub struct ScreenLoc {
    pub sides: Rect<f32>,
}
