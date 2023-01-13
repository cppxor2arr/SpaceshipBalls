use std::time::Duration;

use legion::{system, systems::CommandBuffer};
use macroquad::{
    color::Color,
    experimental::collections::storage,
    input::{self, KeyCode},
};
use rapier2d::dynamics::RigidBodyHandle;

use crate::{
    bullet::spawn_bullet,
    components::{Bullet, ParticleEmitter, Player, PlayerId},
    particles::{spawn_particles, Particles},
    physics::{apply_force, apply_torque, PhysicsSystem},
    random::LcgRng,
    settings::Settings,
};

#[allow(clippy::too_many_arguments)]
#[system(for_each)]
pub fn input(
    &rigid_body_handle: &RigidBodyHandle,
    player_component: &mut Player,
    &color: &Color,
    emitter: &ParticleEmitter,
    cmd: &mut CommandBuffer,
    #[resource] physics_system: &mut PhysicsSystem,
    #[resource] particles: &mut Particles,
    #[resource] rng: &mut LcgRng,
    #[resource] &elapsed: &Duration,
) {
    let player = physics_system
        .rigid_bodies
        .get_mut(rigid_body_handle)
        .unwrap();
    let settings = storage::get::<Settings>().unwrap();
    let player_settings = {
        match player_component.id {
            PlayerId::Player1 => &settings.player1,
            PlayerId::Player2 => &settings.player2,
        }
    };

    if input::is_key_down(player_settings.up) {
        apply_force(player, KeyCode::W, 1.0);
        spawn_particles(player, particles, emitter, rng, elapsed);
    }
    if input::is_key_down(player_settings.down) {
        apply_force(player, KeyCode::S, 1.0);
    }
    if input::is_key_down(player_settings.left) {
        apply_torque(player, 1.0);
    }
    if input::is_key_down(player_settings.right) {
        apply_torque(player, -1.0);
    }

    player_component.remaining_time += elapsed.as_secs_f32();
    if input::is_key_down(player_settings.shoot)
        && player_component.remaining_time >= settings.shoot_cooldown
    {
        cmd.push((
            Bullet {
                id: player_component.id,
            },
            spawn_bullet(physics_system, rigid_body_handle, rng),
            color,
        ));
        player_component.remaining_time = 0.0;
    }
}
