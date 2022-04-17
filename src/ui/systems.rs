use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{bullet::BulletComponent, enemy::EnemyComponent, ui::FrametimeText};

pub fn setup_frametime_ui(mut commands: Commands, font: Handle<Font>) {
    println!("Setting up UI");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::RED,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: " bullets ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::RED,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: " enemies ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::RED,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: " ms/frame ".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::RED,
                            ..Default::default()
                        },
                    },
                    TextSection {
                        value: " fps".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FrametimeText);
}

pub fn update_frametime_system(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FrametimeText>>,
    enemies: Query<&EnemyComponent>,
    bullets: Query<&BulletComponent>,
) {
    let mut text = query.single_mut();

    let mut bullets_count = 1;
    bullets.for_each(|_| bullets_count += 1);
    text.sections[0].value = format!("{}", bullets_count);

    let mut enemies_count = 1;
    enemies.for_each(|_| enemies_count += 1);
    text.sections[2].value = format!("{}", enemies_count);

    let mut ms = time.delta_seconds_f64();
    if let Some(diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME) {
        if let Some(avg) = diag.average() {
            ms = avg;
        }
    }

    text.sections[4].value = format!("{:.2}", ms * 1000.0);

    let mut fps = 0.0;
    if let Some(diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_cur) = diag.value() {
            fps = fps_cur;
        }
    }
    text.sections[6].value = format!("{:.1}", fps);
}
