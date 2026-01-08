use std::ops::{Add, Sub, Div, Mul};
use macros::VectOps;

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vector3::new($x, $y, $z)
    };
}
#[macro_export]
macro_rules! vec4 {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        Vector4::new($x, $y, $z, $w)
    };
}

#[derive(VectOps, Clone, Copy)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {x, y, z}
    }

    pub const fn as_tuple(&self) -> (f32, f32, f32){
        (self.x, self.y, self.z)
    }
    pub const fn as_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
impl From<Vector3> for Vector4 {
    fn from(v: Vector3) -> Vector4 { vec4!(v.x, v.y, v.z, 0.0) }
}

#[derive(VectOps, Clone, Copy)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
impl Vector4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {x, y, z, w}
    }

    pub const fn as_tuple(&self) -> (f32, f32, f32, f32){
        (self.x, self.y, self.z, self.w)
    }
    pub const fn as_array(&self) -> [f32; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

pub struct Triangle(Vector3, Vector3, Vector3);
impl Triangle {
    pub const fn new(x: Vector3, y: Vector3, z: Vector3) -> Self {
        Self(x,y,z)
    }
    pub const fn as_array(&self) -> [[f32;3];3] {
        [
            self.0.as_array(),
            self.1.as_array(),
            self.2.as_array(),
        ]
    }
}