use crate::{boid::Boid, math::Vector2D, physics::KinematicObject};

#[allow(unused)]
use crate::log;

/**
Boids will try to follow the same direction as it's
neighbors

This rule averages the headings with the nearest boids and
nudges our boid in the same direction */
pub fn following_rule(target: &Boid, neighbors: &Vec<Boid>) -> f32 {
    if neighbors.len() == 0 {
        return 0.0;
    }
    let average_velocity = neighbors.iter().map(|b| b.velocity()).sum::<Vector2D>();
    target.towards(&average_velocity)
}
