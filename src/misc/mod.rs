mod components;
mod resources;
mod setup;
mod systems;

pub use self::{
    components::{
        DamageDealtEvent, DamageOverTime, DeathEvent, Health, HitboxComponent, MainCamera,
        MovableComponent, NameComponent,
    },
    resources::{MouseCoords, ScreenLoc},
    setup::MiscSetupPlugin,
    systems::{
        damage_system, die_system, dot_system, health_death_system, move_system, screen_edge_system,
    },
};
