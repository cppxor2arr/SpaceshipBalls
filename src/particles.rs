use std::{f32::consts::PI, time::Duration};

use macroquad::color::Color;
use rapier2d::{
    dynamics::RigidBody,
    na::{Point2, Rotation2, Vector2},
};

use crate::{components::ParticleEmitter, random::LcgRng};

pub type Particles = Vec<Particle>;

pub struct Particle {
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub lifespan: f32,
    pub lifetime: f32,
    pub color: Color,
}

pub fn spawn_particles(
    player: &RigidBody,
    particles: &mut Particles,
    emitter: &ParticleEmitter,
    rng: &mut LcgRng,
    elapsed: Duration,
) {
    let pos = Point2::from(player.position().translation.vector);
    let angle = player.position().rotation.angle() + PI;

    let dt = elapsed.as_secs_f32();
    let num_particles = emitter.rate * dt;

    particles.extend((0..num_particles.round() as u32).map(|x| {
        let vel = player.velocity_at_point(&pos)
            + Rotation2::new(angle + rng.uniform(-0.15, 0.15)) * Vector2::new(10.0, 0.0);

        Particle {
            pos: pos + vel * -(x as f32) / num_particles * dt,
            vel,
            lifespan: emitter.lifespan,
            lifetime: emitter.lifespan + (x as f32 / num_particles - 1.0) * dt,
            color: emitter.color,
        }
    }));
}
