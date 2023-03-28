use nannou::prelude::*;
mod point;
use crate::point::Point;
use std::time::Instant;

struct Model {
    _window: window::Id,
    point: Point,
    last_time: std::time::Instant,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut point = Point::new();
    point.pos.y = 50.0;
    point.velocity.x = 1000.0;
    let last_time = Instant::now();
    Model {
        _window,
        point,
        last_time,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let now = Instant::now();
    let dt = now.duration_since(model.last_time).secs();
    model.last_time = Instant::now();
    let p2 = Point {
        pos: Vec2::ZERO,
        prev_vel: Vec2::ZERO,
        velocity: Vec2::ZERO,
        acceleration: Vec2::ZERO,
        is_kinematic: true,
        mass: 10.0,
        id: 0,
    };
    model.point.acceleration = gravity_to_place(model.point, p2, 1000.0);
    // model.point.velocity.y = 10.0;
    model.point.update(dt);
    // println!("{:?}", model.point.acceleration);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.ellipse().radius(10.0).color(BLACK).xy(model.point.pos);
    draw.ellipse().radius(10.0).color(GRAY).xy(Vec2::ZERO);

    draw.to_frame(app, &frame).unwrap();
}

fn gravity_to_place(p1: Point, p2: Point, g_const: f32) -> Vec2 {
    // ((m1*m2)/r^2)*G
    let distance_x = p2.pos.x - p1.pos.x;
    let distance_y = p2.pos.y - p1.pos.y;
    let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
    // straight light force
    let force = g_const * p1.mass * p2.mass / distance * distance;

    // angle theta
    let theta = (distance_y).atan2(distance_x);

    let force_x = theta.cos() * force;
    let force_y = theta.sin() * force;

    return vec2(force_x, force_y) / p1.mass;
}

fn calculate_energy(point: Point) -> f32 {
    return 0.5
        * point.mass
        * (point.pos - point.prev_vel).length()
        * (point.pos - point.prev_vel).length();
}
