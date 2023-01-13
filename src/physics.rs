use macroquad::input::KeyCode;
use rapier2d::{
    dynamics::{IntegrationParameters, JointSet, RigidBody, RigidBodySet},
    geometry::{BroadPhase, ColliderSet, NarrowPhase},
    na::Vector2,
    pipeline::PhysicsPipeline,
};

pub struct PhysicsSystem {
    pub pipeline: PhysicsPipeline,
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub joints: JointSet,
    pub event_handler: (),
    remaining_time: f32,
}

impl PhysicsSystem {
    pub fn new() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector2::zeros(),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
            event_handler: (),
            remaining_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.remaining_time += dt;
        while self.remaining_time > 0.0 {
            self.pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.rigid_bodies,
                &mut self.colliders,
                &mut self.joints,
                None,
                None,
                &self.event_handler,
            );
            self.remaining_time -= self.integration_parameters.dt();
        }
    }
}

pub fn apply_force(rigid_body: &mut RigidBody, direction: KeyCode, magnitude: f32) {
    let sign = if direction == KeyCode::W { 1.0 } else { -1.0 };
    let force = {
        let dir = rigid_body.position().rotation.into_inner();
        magnitude * sign * Vector2::new(dir.re, dir.im)
    };
    rigid_body.apply_force(force, true);
}

pub fn apply_torque(rigid_body: &mut RigidBody, torque: f32) {
    rigid_body.apply_torque(torque, true);
}
