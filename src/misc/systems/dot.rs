use crate::misc::components::{DamageDealtEvent, DamageOverTime};
use bevy::prelude::*;

pub fn dot_system(
    mut ev_hit: EventWriter<DamageDealtEvent>,
    time: Res<Time>,
    mut query: Query<(&mut DamageOverTime, Entity), With<DamageOverTime>>,
) {
    for (mut dot, entity) in query.iter_mut() {
        if dot.delay.tick(time.delta()).finished() {
            ev_hit.send(DamageDealtEvent {
                entity,
                damage: dot.damage,
            });
            dot.delay.reset();
        }
    }
}
