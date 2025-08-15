use crate::physics::{KinematicObject, Vector2D};
use crate::screen::ScreenObject;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Boid {
    pub x: f32,
    pub y: f32,
    pub s: f32,
    pub a: f32,
}

impl ScreenObject for Boid {
    fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    fn with_position(&self, x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            s: self.s,
            a: self.a,
        }
    }
}

impl KinematicObject for Boid {
    fn get_position_vector(&self) -> Vector2D {
        Vector2D::new(self.x, self.y)
    }

    fn get_velocity_vector(&self) -> Vector2D {
        let (y, x) = self.a.sin_cos();
        self.s * Vector2D::new(x, y)
    }
}
