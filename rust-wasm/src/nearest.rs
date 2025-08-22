use crate::boid::Boid;
use crate::physics::KinematicObject;
use std::iter::FromIterator;

pub fn nearest_boids(max_distance: f32, target: &Boid, objects: &Vec<Boid>) -> Vec<Boid> {
    let nearest_iter = objects
        .iter()
        .filter(|b| (*b) != target)
        .filter(|b| b.position().metric_distance(&target.position()) < max_distance)
        .map(|b| b.clone());
    Vec::from_iter(nearest_iter)
}
