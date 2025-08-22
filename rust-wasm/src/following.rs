use crate::{
    boid::Boid,
    math::{cross_2d, Vector2D},
    physics::KinematicObject,
};

#[allow(unused)]
use crate::log;

/**
Boids will try to follow the same direction as it's
neighbors

This rule averages the headings with the nearest boids and
nudges our boid in the same direction */
pub fn following_rule(target: &Boid, neighbors: &Vec<Boid>) -> f32 {
    let average_heading = neighbors
        .iter()
        .map(|b| b.velocity() / b.velocity().magnitude_squared())
        .sum::<Vector2D>();
    -cross_2d(&average_heading, &target.velocity().normalize())
}
