use macroquad::experimental::collections::storage;
use rapier2d::{
    dynamics::{RigidBodyBuilder, RigidBodyHandle},
    geometry::ColliderBuilder,
    na::{Rotation2, Vector2},
};

use crate::{physics::PhysicsSystem, random::LcgRng, settings::Settings};

pub fn spawn_bullet(
    physics_system: &mut PhysicsSystem,
    rigid_body_handle: RigidBodyHandle,
    rng: &mut LcgRng,
) -> RigidBodyHandle {
    let settings = storage::get::<Settings>().unwrap();
    let player = physics_system
        .rigid_bodies
        .get_mut(rigid_body_handle)
        .unwrap();
    let dir = player.position().rotation
        * Rotation2::new(rng.uniform(-settings.bullet.spread, settings.bullet.spread))
        * Vector2::identity();
    let vel = player.linvel() + settings.bullet.speed * dir;
    let pos = player.position().translation.vector + 0.7 * dir;

    let handle = {
        let body = RigidBodyBuilder::new_dynamic()
            .translation(pos.x, pos.y)
            .linvel(vel.x, vel.y)
            .build();
        physics_system.rigid_bodies.insert(body)
    };

    let collider = ColliderBuilder::ball(settings.bullet.radius)
        .restitution(settings.restitution)
        .density(settings.bullet.density)
        .build();
    physics_system
        .colliders
        .insert(collider, handle, &mut physics_system.rigid_bodies);

    handle
}
