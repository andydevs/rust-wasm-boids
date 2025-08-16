mod boid;
mod generate_boids;
mod nearest;
mod physics;
mod screen;
mod separation;
mod utils;

use boid::Boid;
use generate_boids::generate_boids;
use physics::KinematicObject;
use screen::Screen;
use separation::separation_rule;
use std::iter::FromIterator;
use wasm_bindgen::prelude::*;

use crate::utils::set_panic_hook;

#[wasm_bindgen]
pub struct BoidsSim {
    min_separation: f32,
    max_angle_change: f32,
    screen: Screen,
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl BoidsSim {
    pub fn init(
        width: u16,
        height: u16,
        boid_length: u16,
        min_separation: f32,
        max_angle_change: f32,
        n: u16,
    ) -> Self {
        set_panic_hook();
        Self {
            min_separation: min_separation + (boid_length as f32),
            max_angle_change,
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
            .map(|b| self.screen.handle_teleporting(&b))
            .map(|b| separation_rule(self.min_separation, self.max_angle_change, &b, &self.boids));
        self.boids = Vec::from_iter(new_boids);
    }
}
