use macroquad::experimental::collections::storage;
use rapier2d::{
    dynamics::{RigidBodyBuilder, RigidBodyHandle},
    geometry::ColliderBuilder,
    na::Isometry2,
};

use crate::{physics::PhysicsSystem, settings::Settings};

pub fn spawn_walls(physics_system: &mut PhysicsSystem) -> RigidBodyHandle {
    let handle = {
        let body = RigidBodyBuilder::new_static().build();
        physics_system.rigid_bodies.insert(body)
    };

    let settings = storage::get::<Settings>().unwrap();
    const THICKNESS: f32 = 0.1;

    let collider_top = ColliderBuilder::cuboid(16.0, THICKNESS)
        .translation(0.0, 9.0 - THICKNESS)
        .restitution(settings.restitution)
        .build();
    let collider_left = ColliderBuilder::cuboid(THICKNESS, 9.0)
        .translation(-16.0 + THICKNESS, 0.0)
        .restitution(settings.restitution)
        .build();
    let collider_bottom = ColliderBuilder::cuboid(16.0, THICKNESS)
        .translation(0.0, -9.0 + THICKNESS)
        .restitution(settings.restitution)
        .build();
    let collider_right = ColliderBuilder::cuboid(THICKNESS, 9.0)
        .translation(16.0 - THICKNESS, 0.0)
        .restitution(settings.restitution)
        .build();
    physics_system
        .colliders
        .insert(collider_top, handle, &mut physics_system.rigid_bodies);
    physics_system
        .colliders
        .insert(collider_left, handle, &mut physics_system.rigid_bodies);
    physics_system
        .colliders
        .insert(collider_bottom, handle, &mut physics_system.rigid_bodies);
    physics_system
        .colliders
        .insert(collider_right, handle, &mut physics_system.rigid_bodies);

    handle
}

pub fn spawn_spaceship(
    physics_system: &mut PhysicsSystem,
    position: Isometry2<f32>,
) -> RigidBodyHandle {
    let handle = {
        let body = RigidBodyBuilder::new_dynamic().position(position).build();
        physics_system.rigid_bodies.insert(body)
    };

    let settings = storage::get::<Settings>().unwrap();
    const RADIUS: f32 = 0.05;
    // triangle collider doesn't work yet so using three capsules as workaround
    let collider_left = ColliderBuilder::capsule_x(0.5, RADIUS)
        .restitution(settings.restitution)
        .rotation(-20_f32.to_radians())
        .translation(0.157, 0.171)
        .build();
    let collider_right = ColliderBuilder::capsule_x(0.5, RADIUS)
        .restitution(settings.restitution)
        .rotation(20_f32.to_radians())
        .translation(0.157, -0.171)
        .build();
    let collider_bottom = ColliderBuilder::capsule_y(0.342, RADIUS)
        .restitution(settings.restitution)
        .translation(-0.313, 0.0)
        .build();

    let colliders = &mut physics_system.colliders;
    colliders.insert(collider_left, handle, &mut physics_system.rigid_bodies);
    colliders.insert(collider_right, handle, &mut physics_system.rigid_bodies);
    colliders.insert(collider_bottom, handle, &mut physics_system.rigid_bodies);

    handle
}
