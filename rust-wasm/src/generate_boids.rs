use crate::boid::Boid;
use rand::prelude::*;
use rand::rng;
use std::f32::consts::PI;
use std::iter::FromIterator;

pub fn generate_boids(width: u16, height: u16, boid_velocity: f32, n: u16) -> Vec<Boid> {
    let rx: f32 = (width as f32) / 2.0;
    let ry: f32 = (height as f32) / 2.0;
    let iter = (0..n).scan(rng(), |s, i| {
        let x = (s.random::<f32>() + 1.0) * rx;
        let y = (s.random::<f32>() + 1.0) * ry;
        let a = s.random::<f32>() * 2.0 * PI;
        Some(Boid::new(i, x, y, boid_velocity, a))
    });
    Vec::from_iter(iter)
}
