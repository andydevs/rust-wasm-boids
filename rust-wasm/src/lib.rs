mod boid;
mod generate_boids;
mod physics;
mod screen;
mod utils;

use boid::Boid;
use generate_boids::generate_boids;
use physics::KinematicObject;
use screen::Screen;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BoidsSim {
    screen: Screen,
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl BoidsSim {
    pub fn init(width: u16, height: u16, boid_length: u16, n: u16) -> Self {
        Self {
            screen: Screen {
                width,
                height,
                boundary_offset: boid_length / 2,
            },
            boids: generate_boids(width, height, n),
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
            .map(|b| self.screen.handle_teleporting(&b));
        self.boids = Vec::from_iter(new_boids);
    }
}
