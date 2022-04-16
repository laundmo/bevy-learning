mod components;
mod spawn;
mod systems;

pub use self::{
    components::FrametimeText,
    spawn::spawn_ui,
    systems::{setup_frametime_ui, update_frametime_system},
};
