// console.log
extern crate web_sys;
#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use crate::{
    boid::Boid,
    nearest::nearest_boids,
    physics::{KinematicObject, Vector2D},
    screen::cross_2d,
};
use std::iter::FromIterator;

/**
Boids that are close to each other
will try to move away from each other.

This rule takes a target boid and the given
list of boids and returns an updated boid */
pub fn separation_rule(
    min_separation: f32,
    max_angle_change: f32,
    target: &Boid,
    objects: &Vec<Boid>,
) -> Boid {
    let diff_iter = nearest_boids(min_separation, target, objects)
        .into_iter()
        .map(|p| p.get_position_vector() - target.get_position_vector());
    let diffs = Vec::from_iter(diff_iter);
    let diff_count = (&diffs).iter().count();

    if diff_count > 0 {
        let total_diff = (&diffs)
            .iter()
            .map(|b| b / b.magnitude_squared())
            .sum::<Vector2D>();
        let diff_vector = total_diff.normalize();
        let heading_vector = target.get_velocity_vector().normalize();
        let cross = cross_2d(&diff_vector, &heading_vector);
        if cross < 1e-5 {
            target.clone()
        } else {
            let diff_direction = cross.signum();
            let diff_angle_change = diff_direction * max_angle_change;
            target.with_angle_change(diff_angle_change)
        }
    } else {
        target.clone()
    }
}
