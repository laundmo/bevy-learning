mod dot;
mod health;
mod mouse;
mod movement;
mod screen;

pub use self::{
    dot::dot_system,
    health::{damage_system, die_system, health_death_system},
    mouse::cursor_pos_system,
    movement::move_system,
    screen::screen_edge_system,
};
