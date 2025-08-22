use crate::{boid::Boid, math::Vector2D, physics::KinematicObject};

#[allow(unused)]
use crate::log;

/**
Boids will try to follow the same direction as it's
neighbors

This rule averages the headings with the nearest boids and
nudges our boid in the same direction */
pub fn center_rule(target: &Boid, neighbors: &Vec<Boid>) -> f32 {
    if neighbors.len() == 0 {
        return 0.0;
    }
    let center_of_mass = neighbors
        .iter()
        .map(|b| b.position() - target.position())
        .map(|v| v * v.magnitude())
        .sum::<Vector2D>();
    target.towards(&center_of_mass)
}
