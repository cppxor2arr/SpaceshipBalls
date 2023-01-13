use legion::system;
use macroquad::{color::Color, shapes};
use rapier2d::dynamics::RigidBodyHandle;

use crate::{particles::Particles, physics::PhysicsSystem, renderer};

#[system(for_each)]
pub fn render_base(
    &rigid_body_handle: &RigidBodyHandle,
    &color: &Color,
    #[resource] physics_system: &PhysicsSystem,
) {
    if let Some(rigid_body) = physics_system.rigid_bodies.get(rigid_body_handle) {
        renderer::draw_colliders(
            rigid_body
                .colliders()
                .iter()
                .filter_map(|&handle| physics_system.colliders.get(handle)),
            color,
        );
    }
}

#[system]
#[allow(clippy::ptr_arg)]
pub fn render_particles(#[resource] particles: &Particles) {
    for p in particles {
        shapes::draw_poly(p.pos.x, p.pos.y, 6, 0.1, 0.0, p.color);
    }
}
