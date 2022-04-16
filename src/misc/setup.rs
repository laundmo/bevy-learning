use bevy::prelude::*;

use super::{
    components::MainCamera, damage_system, die_system, dot_system, health_death_system,
    move_system, resources::ScreenLoc, screen_edge_system, systems::cursor_pos_system, MouseCoords,
};

pub struct MiscSetupPlugin;

impl Plugin for MiscSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup_system)
            .insert_resource(MouseCoords {
                pos: Vec2::default(),
            })
            .insert_resource(ScreenLoc::default())
            .add_system(damage_system)
            .add_system(dot_system)
            .add_system(move_system)
            .add_system(cursor_pos_system)
            .add_system(screen_edge_system)
            .add_system(
                health_death_system
                    .after(dot_system)
                    .after(damage_system)
                    .before(die_system),
            )
            .add_system(die_system.after(health_death_system));
    }
}

fn camera_setup_system(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}
