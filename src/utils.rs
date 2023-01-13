use macroquad::{
    camera::{self, Camera2D},
    math::{vec2, Vec2},
    prelude::Conf,
    window,
};
use rapier2d::na::{Point2, Vector2};

use crate::constants::{REL_HEIGHT, REL_WIDTH};

pub fn point2_to_vec2(point: Point2<f32>) -> Vec2 {
    vec2(point.x, point.y)
}

pub fn vector2_to_vec2(vector: Vector2<f32>) -> Vec2 {
    vec2(vector.x, vector.y)
}

pub fn handle_resize() {
    let screen_width_to_height_ratio = window::screen_width() / window::screen_height();
    let zoom = if screen_width_to_height_ratio < REL_WIDTH / REL_HEIGHT {
        vec2(1.0, screen_width_to_height_ratio) / REL_WIDTH
    } else {
        vec2(1.0 / screen_width_to_height_ratio, 1.0) / REL_HEIGHT
    };

    camera::set_camera(Camera2D {
        zoom,
        ..Default::default()
    });
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Space game"),
        fullscreen: false,
        ..Default::default()
    }
}
