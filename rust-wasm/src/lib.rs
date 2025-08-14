mod utils;

use rand::prelude::*;
use rand::rng;
use std::f32::consts::PI;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;
type Vector2D = nalgebra::Vector2<f32>;

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

    pub fn updated_position(&self) -> Self {
        let p0 = self.get_position();
        let (y, x) = self.a.sin_cos();
        let v = BOID_VELOCITY * Vector2D::new(x, y);
        let p1 = p0 + v;
        Self::from_position_angle(&p1, self.a)
    }
}

struct Screen {
    width: u16,
    height: u16,
}

#[wasm_bindgen]
pub struct BoidsSim {
    screen: Screen,
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl BoidsSim {
    pub fn init(width: u16, height: u16, n: u16) -> Self {
        let rx: f32 = f32::from(width) / 2.0;
        let ry: f32 = f32::from(height) / 2.0;
        let iter = (0..n).scan(rng(), |s, _i| {
            let x = (s.random::<f32>() + 1.0) * rx;
            let y = (s.random::<f32>() + 1.0) * ry;
            let a = s.random::<f32>() * 2.0 * PI;
            Some(Boid { x, y, a })
        });
        Self {
            screen: Screen {
                width: width,
                height: height,
            },
            boids: Vec::from_iter(iter),
        }
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn update_boids(&mut self) {
        let new_boids = self.boids.iter().map(|b| b.updated_position());
        self.boids = Vec::from_iter(new_boids);
    }
}
