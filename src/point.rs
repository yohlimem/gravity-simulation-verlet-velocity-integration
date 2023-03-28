use std::ops::Mul;

use nannou::prelude::*;


#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec2,
    pub prev_vel: Vec2,
    pub velocity: Vec2,
    /// acceleration
    pub acceleration: Vec2,
    pub is_kinematic: bool,
    pub mass: f32,
    pub id:u32,
}

impl Point {
    pub fn update(&mut self, dt: f64){
        if self.is_kinematic || self.acceleration.is_nan() {return}
        self.verlet_integration(dt); 
    }
    pub fn new() -> Point{
        Point {
            pos: vec2(0.0, 0.0),
            prev_vel: vec2(0.0, 0.0),
            velocity: vec2(0.0, 0.0),
            // aka acceleration
            acceleration: vec2(0.0, 0.0),
            is_kinematic: false,
            mass: 1.0,
            id: 0,
        }
    }
    fn verlet_integration(&mut self, dt: f64){
        let prev_pos = self.velocity;
        // self.pos = self.pos * 2.0 - self.prev_pos + self.acceleration.mul((dt*dt) as f32);
        let future_acceleration = self.velocity - self.prev_vel;
        println!("velocity: {}", self.velocity + ((self.acceleration + future_acceleration) * dt as f32));
        println!("self.pos: {}", self.pos + self.velocity * dt as f32 + (0.5 * self.acceleration.mul((dt*dt) as f32)));
        println!("future acceleration: {}", future_acceleration);
        println!("acceleration: {}", self.acceleration);
        // https://en.wikipedia.org/wiki/Verlet_integration#Algorithmic_representation
        self.pos = self.pos + self.velocity * dt as f32 + self.acceleration * (dt * dt * 0.5) as f32;
        self.velocity = self.velocity + (self.acceleration+future_acceleration) * (dt*0.5) as f32;
        self.prev_vel = prev_pos;
    }
    // pub fn from(pos: Vec2, velocity: Vec2,acceleration: Vec2,is_kinematic: bool, id: u32) -> Point{
    //     Point {
    //         pos,
    //         velocity,
    //         acceleration,
    //         is_kinematic,
    //         id,
    //     }
    // }
    pub fn air_drag(&mut self){
        self.acceleration = self.acceleration.mul(0.99);
        self.velocity = self.velocity.mul(0.99);
    }
    pub fn gravity(&mut self, dt: f64){
        self.velocity.y += -1.0 * dt as f32;
    }
}