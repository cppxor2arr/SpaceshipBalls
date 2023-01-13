use std::{f32::consts::PI, time::Instant};

use legion::{Resources, Schedule, World};
use macroquad::{
    color,
    experimental::{
        collections::storage,
        scene::{self, Handle, Node, RefMut},
    },
    input::{self, KeyCode},
};
use rapier2d::na::{Isometry2, Vector2};

use crate::{
    components::{ParticleEmitter, Player, PlayerId},
    init_world, input_system,
    particles::Particles,
    pause_menu::PauseMenu,
    physics::PhysicsSystem,
    random::LcgRng,
    render_system, settings, update_system,
};

pub struct Game {
    world: World,
    resources: Resources,
    update_schedule: Schedule,
    render_schedule: Schedule,
    pause_menu_handle: Handle<PauseMenu>,
    instant: Instant,
}

impl Game {
    pub fn new() -> Self {
        Self {
            world: World::default(),
            resources: Resources::default(),
            update_schedule: Schedule::builder()
                .add_system(input_system::input_system())
                .add_system(update_system::update_base_system())
                .add_system(update_system::update_particles_system())
                .build(),
            render_schedule: Schedule::builder()
                .add_system(render_system::render_base_system())
                .add_system(render_system::render_particles_system())
                .build(),
            pause_menu_handle: Handle::null(),
            instant: Instant::now(),
        }
    }
}

impl Node for Game {
    fn ready(mut node: RefMut<Self>)
    where
        Self: Sized,
    {
        storage::store(settings::load_settings());
        let mut physics_system = PhysicsSystem::new();

        node.world
            .push((init_world::spawn_walls(&mut physics_system), color::GREEN));
        node.world.push((
            Player {
                id: PlayerId::Player1,
                remaining_time: 0.0,
            },
            init_world::spawn_spaceship(
                &mut physics_system,
                Isometry2::new(Vector2::new(-11.0, 0.0), 0.0),
            ),
            color::BLUE,
            ParticleEmitter {
                color: color::BLUE,
                lifespan: 0.5,
                rate: 300.0,
            },
        ));
        node.world.push((
            Player {
                id: PlayerId::Player2,
                remaining_time: 0.0,
            },
            init_world::spawn_spaceship(
                &mut physics_system,
                Isometry2::new(Vector2::new(11.0, 0.0), PI),
            ),
            color::RED,
            ParticleEmitter {
                color: color::RED,
                lifespan: 0.5,
                rate: 300.0,
            },
        ));

        node.resources.insert(physics_system);
        node.resources.insert(Particles::new());
        node.resources.insert(LcgRng::new(0));
    }

    fn update(mut node: RefMut<Self>)
    where
        Self: Sized,
    {
        if let Some(pause_node) = scene::get_node(node.pause_menu_handle) {
            if pause_node.active {
                return;
            } else {
                pause_node.delete();
                node.pause_menu_handle = Handle::null();
                node.instant = Instant::now();
            }
        }
        if input::is_key_pressed(KeyCode::GraveAccent) {
            node.pause_menu_handle = scene::add_node(PauseMenu { active: true });
        }
        if input::is_key_pressed(KeyCode::Escape) {
            node.delete();
            return;
        }

        let node = &mut *node;
        node.resources.insert(node.instant.elapsed());
        node.instant = Instant::now();
        node.update_schedule
            .execute(&mut node.world, &mut node.resources);
    }

    fn draw(mut node: RefMut<Self>)
    where
        Self: Sized,
    {
        let node = &mut *node;
        node.render_schedule
            .execute(&mut node.world, &mut node.resources);
    }
}
