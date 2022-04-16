use bevy::prelude::*;

use super::setup_frametime_ui;

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraCodeBold.ttf");

    commands.spawn_bundle(UiCameraBundle::default());
    setup_frametime_ui(commands, font);
}
