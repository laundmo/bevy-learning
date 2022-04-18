use bevy::{math::Vec3Swizzles, prelude::*};

use crate::misc::components::{DamageDealtEvent, DamageOverTime, Health, MovableComponent};

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<DamageDealtEvent>,
    query: Query<&Transform, With<Health>>,
) {
    let font = asset_server.load("FiraCodeBold.ttf");
    for evt in reader.iter() {
        if evt.silent {
            return;
        }
        if let Ok(&transform) = query.get(evt.entity) {
            commands
                .spawn()
                .insert(MovableComponent {
                    heading: Vec3::new(0.0, 1.0, 0.0),
                    speed: 1.0,
                })
                .insert(Health {
                    value: 5.0,
                    ..default()
                })
                .insert(DamageOverTime { ..default() })
                .insert_bundle(Text2dBundle {
                    transform: Transform {
                        translation: transform.translation.xy().extend(1.0),
                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: format!("{}", evt.damage),
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 20.0,
                                color: Color::RED,
                                ..default()
                            },
                        }],
                        ..default()
                    },
                    ..default()
                });
        }
    }
}
