use macroquad::{color::Color, math::Vec2, shapes};
use rapier2d::{
    geometry::{Collider, ShapeType},
    na::Vector2,
};

use crate::utils;

pub fn draw_colliders<'a, I>(colliders: I, color: Color)
where
    I: IntoIterator<Item = &'a Collider>,
{
    for collider in colliders {
        (match collider.shape().shape_type() {
            ShapeType::Capsule => draw_capsule,
            ShapeType::Cuboid => draw_cuboid,
            ShapeType::Ball => draw_ball,
            _ => unimplemented!(),
        })(collider, color);
    }
}

fn draw_ball(collider: &Collider, color: Color) {
    let translation = collider.position().translation;
    let radius = collider.shape().as_ball().unwrap().radius;

    shapes::draw_circle(translation.x, translation.y, radius, color);
}

fn draw_capsule(collider: &Collider, color: Color) {
    let position = collider.position();
    let capsule = collider.shape().as_capsule().unwrap();
    let segment = capsule.segment.transformed(position);
    let normal = capsule.radius * segment.normal().unwrap().into_inner();
    let p1 = utils::point2_to_vec2(segment.a + normal);
    let p2 = utils::point2_to_vec2(segment.b + normal);
    let p3 = utils::point2_to_vec2(segment.b - normal);
    let p4 = utils::point2_to_vec2(segment.a - normal);

    shapes::draw_circle(segment.a.x, segment.a.y, capsule.radius, color);
    draw_rectangle(p1, p2, p3, p4, color);
    shapes::draw_circle(segment.b.x, segment.b.y, capsule.radius, color);
}

fn draw_cuboid(collider: &Collider, color: Color) {
    let position = collider.position();
    let cuboid = collider.shape().as_cuboid().unwrap();
    let half_extent1 = position.rotation * Vector2::new(cuboid.half_extents.x, 0.0);
    let half_extent2 = position.rotation * Vector2::new(0.0, cuboid.half_extents.y);
    let p1 = utils::vector2_to_vec2(position.translation.vector + half_extent1 + half_extent2);
    let p2 = utils::vector2_to_vec2(position.translation.vector + -half_extent1 + half_extent2);
    let p3 = utils::vector2_to_vec2(position.translation.vector + -half_extent1 - half_extent2);
    let p4 = utils::vector2_to_vec2(position.translation.vector + half_extent1 - half_extent2);

    draw_rectangle(p1, p2, p3, p4, color);
}

fn draw_rectangle(p1: Vec2, p2: Vec2, p3: Vec2, p4: Vec2, color: Color) {
    shapes::draw_triangle(p1, p2, p3, color);
    shapes::draw_triangle(p3, p4, p1, color);
}
