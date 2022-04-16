use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use misc::{DamageDealtEvent, DeathEvent};

mod bullet;
mod damagenums;
mod enemy;
mod misc;
mod player;
mod ui;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::Immediate,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(misc::MiscSetupPlugin)
        .add_plugin(HelloPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_event::<DamageDealtEvent>()
        .add_event::<DeathEvent>()
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player::spawn_player)
            .add_startup_system(ui::spawn_ui)
            .add_system(ui::update_frametime_system)
            .add_system(enemy::spawn_enemies_system)
            .add_system(enemy::enemy_targeting_system)
            .add_system(player::player_movement_system)
            .add_system(player::player_fire_gun)
            .add_system(bullet::collide_bullet)
            .add_system(bullet::collide_wall)
            .add_system(
                damagenums::spawn
                    .before(misc::die_system)
                    .after(misc::damage_system),
            )
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}
