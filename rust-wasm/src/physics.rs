use crate::{math::Vector2D, screen::ScreenObject};

pub trait KinematicObject: ScreenObject + Sized {
    fn velocity(&self) -> Vector2D;

    fn position(&self) -> Vector2D {
        let (x, y) = self.screen_coords();
        Vector2D::new(x, y)
    }

    fn heading(&self) -> Vector2D {
        self.velocity().normalize()
    }

    fn relative_to<K: KinematicObject>(&self, other: &K) -> Vector2D {
        self.position() - other.position()
    }

    fn move_position_with_velocity(&self, dt: f32) -> Self {
        let p0 = self.position();
        let p1 = p0 + self.velocity() * dt;
        self.with_screen_coords(p1[0], p1[1])
    }
}
