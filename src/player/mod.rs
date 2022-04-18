mod components;
mod spawn;
mod systems;

pub use self::{components::PlayerComponent, spawn::spawn_player, systems::player_movement_system};
