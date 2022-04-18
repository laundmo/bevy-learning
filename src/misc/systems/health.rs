use crate::misc::components::{DamageDealtEvent, DeathEvent, Health};
use bevy::prelude::*;

pub fn damage_system(
    mut reader: EventReader<DamageDealtEvent>,
    mut query: Query<&mut Health, With<Health>>,
) {
    for evt in reader.iter() {
        if let Ok(mut alive) = query.get_mut(evt.entity) {
            alive.value -= evt.damage;
        }
    }
}

pub fn health_death_system(
    mut ev_death: EventWriter<DeathEvent>,
    mut commands: Commands,
    mut query: Query<(&Health, Entity)>,
) {
    for (health, entity) in query.iter_mut() {
        if health.value <= 0.001 {
            commands.entity(entity).remove::<Health>();
            ev_death.send(DeathEvent { entity });
        }
    }
}

pub fn die_system(mut reader: EventReader<DeathEvent>, mut commands: Commands) {
    for evt in reader.iter() {
        commands.entity(evt.entity).despawn_recursive();
    }
}
