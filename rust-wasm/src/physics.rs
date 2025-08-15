use crate::screen::ScreenObject;
pub type Vector2D = nalgebra::Vector2<f32>;

pub trait KinematicObject: ScreenObject + Sized {
    fn get_position_vector(&self) -> Vector2D;
    fn get_velocity_vector(&self) -> Vector2D;

    fn move_position_with_velocity(&self) -> Self {
        let p0 = self.get_position_vector();
        let p1 = p0 + self.get_velocity_vector();
        self.with_position(p1[0], p1[1])
    }
}
