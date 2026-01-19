pub mod vectors;
pub mod colors;
pub mod matrix;

pub use vectors::{Vector, Vector3, Vector4, Quaternion};
pub use matrix::{Matrix, Matrix4x4};
pub use colors::Color;

pub fn perspective(fov_deg: f32, aspect: f32, near: f32, far: f32) -> Matrix4x4 {
    let scaling_factor = 
        1.0 / (fov_deg.to_radians()/2.0).tan();
    Matrix4x4::from_arrays([
        [scaling_factor / aspect, 0.0, 0.0, 0.0],
        [0.0, scaling_factor, 0.0, 0.0],
        [0.0, 0.0, (far+near)/(near-far), (2.0*far*near)/(near-far)],
        [0.0, 0.0, -1.0, 0.0],
    ])
}
