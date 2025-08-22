mod boid;
mod following;
mod generate_boids;
mod math;
mod nearest;
mod physics;
mod screen;
mod separation;
mod utils;

use boid::Boid;
use generate_boids::generate_boids;
use nearest::nearest_boids;
use physics::KinematicObject;
use screen::Screen;
use separation::separation_rule;
use std::iter::FromIterator;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

use crate::following::following_rule;

#[wasm_bindgen]
pub struct BoidsSim {
    max_query_distance: f32,
    max_angle_change: f32,
    rule_weights: [f32; 2],
    screen: Screen,
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl BoidsSim {
    pub fn init(
        width: u16,
        height: u16,
        boid_length: u16,
        max_query_distance: f32,
        max_angle_change: f32,
        separation_weight: f32,
        following_weight: f32,
        n: u16,
    ) -> Self {
        set_panic_hook();
        Self {
            max_query_distance: max_query_distance + (boid_length as f32),
            max_angle_change,
            rule_weights: [separation_weight, following_weight],
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
            .map(|boid| boid.move_position_with_velocity())
            .map(|boid| self.screen.handle_teleporting(&boid))
            .map(|boid| {
                let neighbors = nearest_boids(self.max_query_distance, &boid, &self.boids);
                let rule_outputs = [
                    separation_rule(&boid, &neighbors),
                    following_rule(&boid, &neighbors),
                ];
                let total_angle_factor = rule_outputs
                    .iter()
                    .zip(self.rule_weights)
                    .map(|(rule, weight)| rule * weight)
                    .sum::<f32>()
                    / self.rule_weights.iter().sum::<f32>();
                boid.with_angle_change(total_angle_factor * self.max_angle_change)
            });
        self.boids = Vec::from_iter(new_boids);
    }
}
