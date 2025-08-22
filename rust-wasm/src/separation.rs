use crate::{
    boid::Boid,
    math::{cross_2d, Vector2D},
    physics::KinematicObject,
};

#[allow(unused)]
use crate::log;

/**
Boids that are close to each other
will try to move away from each other.

This rule takes a target boid and its nearest neighbors
and returns the amount of angular velocity it'll impart
on the boid (as a factor of its maximum) */
pub fn separation_rule(target: &Boid, neighbors: &Vec<Boid>, separation: f32) -> f32 {
    if neighbors.len() == 0 {
        return 0.0;
    }
    let diff = neighbors
        .into_iter()
        .map(|p| p.relative_to(target))
        .map(|d| d.normalize() * (d.magnitude() - separation))
        .sum::<Vector2D>();
    cross_2d(&diff, &target.velocity())
}
