use crate::boid::Boid;
use crate::physics::KinematicObject;
use std::iter::FromIterator;

pub fn nearest_boids(min_distance: f32, target: &Boid, objects: &Vec<Boid>) -> Vec<Boid> {
    let nearest_iter = objects
        .iter()
        .filter(|b| {
            if (*b) != target {
                let diff = b.get_position_vector() - target.get_position_vector();
                diff.magnitude() < min_distance
            } else {
                false
            }
        })
        .map(|b| b.clone());
    Vec::from_iter(nearest_iter)
}
