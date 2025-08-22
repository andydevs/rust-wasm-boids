use nalgebra::Vector3;

pub type Vector2D = nalgebra::Vector2<f32>;

pub fn cross_2d(a: &Vector2D, b: &Vector2D) -> f32 {
    let a3 = Vector3::<f32>::new(a[0], a[1], 0.0);
    let b3 = Vector3::<f32>::new(b[0], b[1], 0.0);
    a3.cross(&b3)[2]
}
