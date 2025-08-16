use nalgebra::Vector3;

use crate::physics::Vector2D;

pub trait ScreenObject {
    fn get_position(&self) -> (f32, f32);
    fn with_position(&self, x: f32, y: f32) -> Self;
}

pub fn teleport_dimension(length: u16, offset: u16, x: f32) -> f32 {
    let low = -(offset as f32);
    let high = (length + offset) as f32;
    let fl = length as f32;
    if x < low || x > high {
        fl - x
    } else {
        x
    }
}

pub struct Screen {
    pub(crate) width: u16,
    pub(crate) height: u16,
    pub(crate) boundary_offset: u16,
}

impl Screen {
    pub fn handle_teleporting<T: ScreenObject>(&self, obj: &T) -> T {
        let (x, y) = obj.get_position();
        let px = teleport_dimension(self.width, self.boundary_offset, x);
        let py = teleport_dimension(self.height, self.boundary_offset, y);
        return obj.with_position(px, py);
    }
}

pub fn cross_2d(a: &Vector2D, b: &Vector2D) -> f32 {
    let a3 = Vector3::<f32>::new(a[0], a[1], 0.0);
    let b3 = Vector3::<f32>::new(b[0], b[1], 0.0);
    a3.cross(&b3)[2]
}
