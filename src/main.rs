use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::PresentMode};
use misc::components::MainCamera;

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
        .add_plugin(HelloPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_event::<misc::components::DamageDealtEvent>()
        .add_event::<misc::components::DeathEvent>()
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(player::spawn_player)
            .add_startup_system(camera_setup_system)
            .insert_resource(misc::resources::MouseCoords {
                pos: Vec2::default(),
            })
            .insert_resource(misc::resources::ScreenLoc::default())
            .add_system(misc::systems::damage_system)
            .add_system(misc::systems::dot_system)
            .add_system(misc::systems::move_system)
            .add_system(misc::systems::cursor_pos_system)
            .add_system(misc::systems::screen_edge_system)
            .add_system(
                misc::systems::health_death_system
                    .after(misc::systems::dot_system)
                    .after(misc::systems::damage_system)
                    .before(misc::systems::die_system),
            )
            .add_system(misc::systems::die_system.after(misc::systems::health_death_system))
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
                    .before(misc::systems::die_system)
                    .after(misc::systems::damage_system),
            )
            .add_system(bevy::input::system::exit_on_esc_system);
    }
}

fn camera_setup_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
