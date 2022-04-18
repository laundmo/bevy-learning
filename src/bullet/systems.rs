use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::bullet::BulletComponent;
use crate::enemy::components::EnemyComponent;
use crate::misc::{
    components::{DamageDealtEvent, Health, HitboxComponent},
    resources::ScreenLoc,
};
use bevy::math::Vec3Swizzles;

pub fn collide_bullet(
    mut ev_hit: EventWriter<DamageDealtEvent>,
    mut enemy_query: Query<
        (Entity, &mut HitboxComponent, &mut Transform),
        (With<EnemyComponent>, Without<BulletComponent>),
    >,
    bullet_query: Query<
        (&mut BulletComponent, &mut HitboxComponent, &mut Transform),
        (With<BulletComponent>, Without<EnemyComponent>),
    >,
) {
    for (bullet, bullet_hitbox, bullet_ray) in bullet_query.iter() {
        for (entity, enemy_hitbox, enemy_ray) in enemy_query.iter_mut() {
            let etrans = enemy_ray.translation;
            let esize = enemy_hitbox.scale;
            let btrans = bullet_ray.translation;
            let bsize = bullet_hitbox.scale;

            // TODO: optimise: check distance beforehand

            if collide(etrans, esize, btrans, bsize).is_some() {
                ev_hit.send(DamageDealtEvent {
                    entity,
                    silent: false,
                    damage: bullet.damage,
                });
            }
        }
    }
}

// TODO: make custom component for things that can die from wall contact
pub fn collide_wall(
    mut ev_death: EventWriter<DamageDealtEvent>,
    screen: Res<ScreenLoc>,
    mut bullet_query: Query<(&Health, &Transform, Entity), With<BulletComponent>>,
) {
    for (health, pos, entity) in bullet_query.iter_mut() {
        let [x, y] = pos.translation.xy().to_array();
        if y < screen.sides.bottom
            || x < screen.sides.left
            || y > screen.sides.top
            || x > screen.sides.right
        {
            ev_death.send(DamageDealtEvent {
                entity,
                silent: true,
                damage: health.value,
            });
        }
    }
}
