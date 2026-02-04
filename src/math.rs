pub mod color;
pub mod vector;
pub mod matrix;
pub mod quaternion;

pub use color::Color;
pub use vector::{Vector, Vector3, Vector4};
pub use matrix::{Matrix, Mat3, Mat4};
pub use quaternion::Quaternion;

pub fn perspective(fov_deg: f32, aspect: f32, near: f32, far: f32) -> Mat4 {
    let scaling_factor = 
        1.0 / (fov_deg.to_radians()/2.0).tan();
    
    Mat4::from_arrays([
        [scaling_factor/aspect, 0.0, 0.0, 0.0],
        [0.0, scaling_factor, 0.0, 0.0],
        [0.0, 0.0, (far+near)/(near-far), (2.0*far*near)/(near-far)],
        [0.0, 0.0, -1.0, 0.0]
    ])
}