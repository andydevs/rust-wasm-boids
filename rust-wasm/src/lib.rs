mod boid;
mod center;
mod following;
mod generate_boids;
mod math;
mod nearest;
mod physics;
mod screen;
mod separation;
mod utils;

use boid::Boid;
use center::center_rule;
use following::following_rule;
use generate_boids::generate_boids;
use nearest::nearest_boids;
use physics::KinematicObject;
use screen::Screen;
use separation::separation_rule;
use std::iter::FromIterator;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BoidsSim {
    max_query_distance: f32,
    max_angle_change: f32,
    separation: f32,
    screen: Screen,
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl BoidsSim {
    pub fn init(
        width: u16,
        height: u16,
        boid_length: u16,
        boid_velocity: f32,
        max_query_distance: f32,
        max_angle_change: f32,
        separation: f32,
        n: u16,
    ) -> Self {
        set_panic_hook();
        Self {
            max_query_distance: max_query_distance + (boid_length as f32),
            max_angle_change,
            separation: separation + (boid_length as f32),
            screen: Screen {
                width,
                height,
                boundary_offset: boid_length / 2,
            },
            boids: generate_boids(width, height, boid_velocity, n),
        }
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn update_boids(&mut self, dt: f32) {
        let new_boids = self
            .boids
            .iter()
            .map(|boid| boid.move_position_with_velocity(dt))
            .map(|boid| self.screen.handle_teleporting(&boid))
            .map(|boid| {
                let neighbors = nearest_boids(self.max_query_distance, &boid, &self.boids);
                let rule_outputs = [
                    separation_rule(&boid, &neighbors, self.separation),
                    following_rule(&boid, &neighbors),
                    center_rule(&boid, &neighbors),
                ];
                let total_angle_factor = rule_outputs.iter().sum::<f32>();
                boid.with_angle_change(total_angle_factor * self.max_angle_change * dt)
            });
        self.boids = Vec::from_iter(new_boids);
    }
}
