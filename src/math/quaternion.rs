use crate::{
    math::{Mat3, Mat4, Vector, Vector3, Vector4},
    vector
};

// #[macro_export]
// macro_rules! quat {
//     ($angle: expr, $x: expr, $y: expr, $z: expr) => {
//         Quaternion::from_angle_vect(angle, Vector3::new([x, y, z]))
//     };
// }

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Quaternion {
    pub const IDENTITY: Self = Self {w: 1.0, x: 0.0, y: 0.0, z: 0.0};
    //bad name
    /// creates a quaternion from axes and one angle NOT RADIAN
    pub fn from_angle_vect(angle: f32, v: Vector3) -> Self {
        Self::from(vector!(angle.to_radians(), v[0], v[1], v[2]))
    }
}
impl From<Vector4> for Quaternion {
    fn from(v: Vector4) -> Self {
        let a = v[0]; // angle I think?

        let axis: Vector3 = vector!(v[1], v[2], v[3]);
        if axis.length()==0.0 {
            return Self { w: 1.0, x: 0.0, y: 0.0, z: 0.0 };
        }
        let n = v.normalized(); //normalized axis

        return Self {
            w: (a/2.0).cos(),
            x: n[0] * (a/2.0).sin(),
            y: n[1] * (a/2.0).sin(),
            z: n[2] * (a/2.0).sin()
        };
    }
}

impl std::ops::Mul for Quaternion {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Self {
            w: self.w*other.w - self.x*other.x - self.y*other.y - self.z*other.z,
            x: self.w*other.x + self.x*other.w + self.y*other.z - self.z*other.y,
            y: self.w*other.y - self.x*other.z + self.y*other.w + self.z*other.x,
            z: self.w*other.z + self.x*other.y - self.y*other.x + self.z*other.w,
        }
    }
}

// impl std::ops::Mul<Vector4> for Quaternion {
//     type Output = Mat4;
//     fn mul(self, rhs: Vector4) -> Self::Output {
//         Mat4::from(self)+rhs
//     }
// }

impl From<Quaternion> for (f32, f32, f32, f32) {
    /// (w, x, y, z)
    fn from(value: Quaternion) -> Self {
        (value.w, value.x, value.y, value.z)
    }
}

impl From<Quaternion> for Mat4 {
    fn from(value: Quaternion) -> Self {
        let (w, x, y, z) = value.into();
        Mat4::from_arrays([
            [
                1.0 - 2.0*(y*y + z*z),
                2.0*(x*y - z*w),
                2.0*(x*z + y*w),
                0.0
            ],
            [
                2.0*(x*y + z*w),
                1.0 - 2.0*(x*x + z*z),
                2.0*(y*z - x*w),
                0.0,
            ],
            [
                2.0*(x*z - y*w),
                2.0*(y*z + x*w),
                1.0 - 2.0*(x*x + y*y),
                0.0,
            ],
            [
                0.0,
                0.0,
                0.0,
                1.0
            ]
        ])
    }
}

impl From<Quaternion> for Mat3 {
    fn from(value: Quaternion) -> Self {
        let (w, x, y, z) = value.into();
        Mat3::from_arrays([
            [
                1.0 - 2.0*(y*y + z*z),
                2.0*(x*y - z*w),
                2.0*(x*z + y*w)
            ],
            [
                2.0*(x*y + z*w),
                1.0 - 2.0*(x*x + z*z),
                2.0*(y*z - x*w)
            ],
            [
                2.0*(x*z - y*w),
                2.0*(y*z + x*w),
                1.0 - 2.0*(x*x + y*y)
            ]
        ])
    }
}