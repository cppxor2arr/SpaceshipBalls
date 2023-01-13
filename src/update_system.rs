use std::time::Duration;

use legion::system;

use crate::{particles::Particles, physics::PhysicsSystem};

#[system]
pub fn update_base(
    #[resource] physics_system: &mut PhysicsSystem,
    #[resource] &elapsed: &Duration,
) {
    physics_system.update(elapsed.as_secs_f32());
}

#[system]
pub fn update_particles(#[resource] particles: &mut Particles, #[resource] &elapsed: &Duration) {
    let dt = elapsed.as_secs_f32();

    for p in &mut *particles {
        p.pos += p.vel * dt;
        p.lifetime -= dt;
        p.color.a = p.lifetime / p.lifespan;
    }

    particles.retain(|p| p.lifetime > 0.0);
}
