use crate::boid::Boid;
use rand::prelude::*;
use rand::rng;
use std::f32::consts::PI;
use std::iter::FromIterator;

pub const BOID_VELOCITY: f32 = 5.0;

pub fn generate_boids(width: u16, height: u16, n: u16) -> Vec<Boid> {
    let rx: f32 = (width as f32) / 2.0;
    let ry: f32 = (height as f32) / 2.0;
    let iter = (0..n).scan(rng(), |s, _i| {
        let x = (s.random::<f32>() + 1.0) * rx;
        let y = (s.random::<f32>() + 1.0) * ry;
        let a = s.random::<f32>() * 2.0 * PI;
        Some(Boid {
            x,
            y,
            s: BOID_VELOCITY,
            a,
        })
    });
    Vec::from_iter(iter)
}
