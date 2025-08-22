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
pub fn separation_rule(target: &Boid, neighbors: &Vec<Boid>) -> f32 {
    let diffs = neighbors.into_iter().map(|p| p.relative_to(target));
    if diffs.clone().count() == 0 {
        return 0.0;
    }
    let diff = diffs.map(|d| d / d.magnitude_squared()).sum::<Vector2D>();
    let correlation = diff.dot(&target.velocity()).max(0.0);
    let turn_dir = cross_2d(&diff, &target.velocity()).signum();
    turn_dir * (diff.magnitude() + correlation)
}
