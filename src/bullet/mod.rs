mod components;
mod spawn;
mod systems;

pub use self::{
    components::BulletComponent,
    spawn::spawn_bullet,
    systems::{collide_bullet, collide_wall},
};
