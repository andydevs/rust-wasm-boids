use crate::math::{cross_2d, Vector2D};
use crate::physics::KinematicObject;
use crate::screen::ScreenObject;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct Boid {
    pub id: u16,
    pub x: f32,
    pub y: f32,
    pub s: f32,
    pub a: f32,
}

impl Boid {
    pub fn new(id: u16, x: f32, y: f32, s: f32, a: f32) -> Self {
        Self { id, x, y, s, a }
    }
    pub fn with_angle_change(&self, da: f32) -> Self {
        Self {
            id: self.id,
            x: self.x,
            y: self.y,
            s: self.s,
            a: self.a + da,
        }
    }
    pub fn towards(&self, direction: &Vector2D) -> f32 {
        let towards = cross_2d(&self.heading(), &direction.normalize());
        if towards.is_normal() {
            towards
        } else {
            0.0
        }
    }
}

impl PartialEq for Boid {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl ScreenObject for Boid {
    fn screen_coords(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn with_screen_coords(&self, x: f32, y: f32) -> Self {
        Self {
            id: self.id,
            x,
            y,
            s: self.s,
            a: self.a,
        }
    }
}

impl KinematicObject for Boid {
    fn velocity(&self) -> Vector2D {
        let (y, x) = self.a.sin_cos();
        self.s * Vector2D::new(x, y)
    }
}
