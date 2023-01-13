use macroquad::color::Color;

pub struct Player {
    pub id: PlayerId,
    pub remaining_time: f32,
}

pub struct Bullet {
    pub id: PlayerId,
}

#[derive(Clone, Copy)]
pub enum PlayerId {
    Player1,
    Player2,
}

pub struct ParticleEmitter {
    pub color: Color,
    pub lifespan: f32,
    pub rate: f32,
}
