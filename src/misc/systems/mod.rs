mod alive;
mod dot;
mod mouse;
mod movement;
mod screen;

pub use self::{
    alive::{damage_system, die_system, health_death_system},
    dot::dot_system,
    mouse::cursor_pos_system,
    movement::move_system,
    screen::screen_edge_system,
};
