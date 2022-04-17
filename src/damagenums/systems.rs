use bevy::prelude::*;

use crate::misc::components::{DamageDealtEvent, DamageOverTime, Health, MovableComponent};

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut reader: EventReader<DamageDealtEvent>,
    query: Query<&Transform, With<Health>>,
) {
    let font = asset_server.load("FiraCodeBold.ttf");
    for evt in reader.iter() {
        if let Ok(&transform) = query.get(evt.entity) {
            commands
                .spawn()
                .insert(MovableComponent {
                    heading: Vec3::new(0.0, 1.0, 0.0),
                    speed: 0.1,
                })
                .insert(Health {
                    value: 3.0,
                    ..default()
                })
                .insert(DamageOverTime { ..default() })
                .insert_bundle(Text2dBundle {
                    transform: Transform {
                        translation: transform.translation,
                        ..default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            value: "12".to_string(), //format!("d {}", evt.damage),
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
