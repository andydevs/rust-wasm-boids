mod utils;

use rand::prelude::*;
use rand::rng;
use std::f32::consts::PI;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;
type Vector2D = nalgebra::Vector2<f32>;

pub struct Screen {
    width: u16,
    height: u16,
}

fn teleport_dimension(length: u16, offset: u16, x: f32) -> f32 {
    let low = -(offset as f32);
    let high = (length + offset) as f32;
    let fl = length as f32;
    if x < low || x > high {
        fl - x
    } else {
        x
    }
}

const BOID_VELOCITY: f32 = 5.0;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub a: f32,
}

impl Boid {
    pub fn get_position(&self) -> Vector2D {
        Vector2D::new(self.x, self.y)
    }

    pub fn from_position_angle(p: &Vector2D, a: f32) -> Self {
        Self {
            x: p[0],
            y: p[1],
            a,
        }
    }

    pub fn move_position_with_velocity(&self) -> Self {
        let p0 = self.get_position();
        let (y, x) = self.a.sin_cos();
        let v = BOID_VELOCITY * Vector2D::new(x, y);
        let p1 = p0 + v;
        Self::from_position_angle(&p1, self.a)
    }

    pub fn handle_screen_teleporting(&self, s: &Screen, offset: u16) -> Self {
        let xp = teleport_dimension(s.width, offset, self.x);
        let yp = teleport_dimension(s.height, offset, self.y);
        Self {
            x: xp,
            y: yp,
            a: self.a,
        }
    }
}

#[wasm_bindgen]
pub struct BoidsSim {
    boid_length: u16,
    screen: Screen,
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl BoidsSim {
    pub fn init(width: u16, height: u16, boid_length: u16, n: u16) -> Self {
        let rx: f32 = f32::from(width) / 2.0;
        let ry: f32 = f32::from(height) / 2.0;
        let iter = (0..n).scan(rng(), |s, _i| {
            let x = (s.random::<f32>() + 1.0) * rx;
            let y = (s.random::<f32>() + 1.0) * ry;
            let a = s.random::<f32>() * 2.0 * PI;
            Some(Boid { x, y, a })
        });
        Self {
            boid_length,
            screen: Screen { width, height },
            boids: Vec::from_iter(iter),
        }
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn update_boids(&mut self) {
        let new_boids = self
            .boids
            .iter()
            .map(|b| b.move_position_with_velocity())
            .map(|b| b.handle_screen_teleporting(&self.screen, self.boid_length / 2));
        self.boids = Vec::from_iter(new_boids);
    }
}
