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

pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl Vector3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self {x, y, z}
    }
}
impl From<Vector3> for Vector4 {
    fn from(v: Vector3) -> Vector4 { vec4!(v.x, v.y, v.z, 0) }
}

pub struct Vector4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32
}
impl Vector4 {
    pub fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self {x, y, z, w}
    }
}