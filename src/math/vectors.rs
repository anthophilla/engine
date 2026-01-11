use macros::VectOps;

#[macro_export]
macro_rules! vector {
    ($x:expr, &y:expr) => {
        Vector2::new($x, $y)
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vector3::new($x, $y, $z)
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        Vector4::new($x, $y, $z, $w)
    };
}

// trait Vector {
//     fn normalize(&self) -> Self;
//     fn dot(&self, other: Self) -> f32;
// }

#[derive(VectOps, Clone, Copy)]
pub struct Vector2 {
    x: f32,
    y: f32,
}
impl Vector2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self {x, y}
    }
    pub const fn as_tuple(&self) -> (f32, f32){
        (self.x, self.y)
    }
    pub const fn as_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x/length,
            y: self.y/length,
        }
    }
    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y*other.y)
    }
    pub fn cross(&self, other: Self) -> f32 {
        self.x*other.y - self.y * other.x
    }
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
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x/length,
            y: self.y/length,
            z: self.z/length,
        }
    }
    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y*other.y) + (self.z*other.z)
    }
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y*other.z - self.z*other.y, 
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x
        }
    }
}
impl From<Vector3> for Vector4 {
    fn from(v: Vector3) -> Vector4 { vector!(v.x, v.y, v.z, 0.0) }
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
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self {
            x: self.x/length,
            y: self.y/length,
            z: self.z/length,
            w: self.w/length,
        }
    }
    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y*other.y) + (self.z*other.z) * (self.w*other.w)
    }
}