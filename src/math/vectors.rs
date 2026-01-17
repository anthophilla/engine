use crate::math::{Matrix, matrix::{Matrix3x3, Matrix4x4}};

#[macro_export]
macro_rules! vector {
    ($x:expr, $y:expr) => {
        Vector::<2>::new([$x, $y])
    };
    ($x:expr, $y:expr, $z:expr) => {
        Vector::<3>::new([$x, $y, $z])
    };
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        Vector::<4>::new([$x, $y, $z, $w])
    };
}

pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vector<const N: usize>(pub [f32; N]);
impl<const N: usize> Vector<N> {
    pub const fn new(v: [f32; N]) -> Self { Self(v) }
    pub const fn as_array(&self) -> [f32; N] { self.0 }
    // square root of (x^2+y^2+z^2...)
    pub fn length(&self) -> f32 {
        self.0.iter().map(|x| x*x).sum::<f32>().sqrt()
    }
    // every component divided by the vectors length
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self(std::array::from_fn(|x| self.0[x]/length))
    }
    // ax*bx + ay*by + ...
    pub fn dot(&self, other: &Self) -> f32 {
        (0..N).map(|i| self.0[i]*other.0[i]).sum()
    }
}

// scalar operators 
impl<const N: usize> std::ops::Add for Vector<N> {
    type Output = Self;
    //add every component together
    fn add(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] + other.0[i]))
    }
}
impl<const N: usize> std::ops::Sub for Vector<N> {
    type Output = Self;
    //add every component together
    fn sub(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] - other.0[i]))
    }
}
impl<const N: usize> std::ops::Mul for Vector<N> {
    type Output = Self;
    //add every component together
    fn mul(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] * other.0[i]))
    }
}
impl<const N: usize> std::ops::Div for Vector<N> {
    type Output = Self;
    //add every component together
    fn div(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self.0[i] / other.0[i]))
    }
}

impl Vector4 {
    pub fn scale(self, scale: Vector3) -> Self {
        Matrix4x4::from_arrays([
            [scale.0[0], 0.0, 0.0, 0.0],
            [0.0, scale.0[1], 0.0, 0.0],
            [0.0, 0.0, scale.0[2], 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])*self
    }
    pub fn translate(self, trans: Vector3) -> Self {
        Matrix4x4::from_arrays([
            [0.0, 0.0, 0.0, trans.0[0]],
            [0.0, 0.0, 0.0, trans.0[1]],
            [0.0, 0.0, 0.0, trans.0[2]],
            [0.0, 0.0, 0.0, 1.0],
        ])*self
    }

    pub fn rotate(self, r: Quaternion) -> Self {
        r.to_matrix4x4()*self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Quaternion {
    //a => angle
    pub fn from(vect: Vector4) -> Self {
        let a = vect.0[0];
        let axis = Vector3::new([vect.0[1], vect.0[2], vect.0[3]]);
        let v = if axis.length()==1.0 {axis} else {axis.normalize()};
        Self{
            w: (a/2.0).cos(),
            x: v.0[0] * (a/2.0).sin(),
            y: v.0[1] * (a/2.0).sin(),
            z: v.0[2] * (a/2.0).sin(),
        }
    }
    pub fn from_radian_vect(angle: f32, v: Vector3) -> Self {
        Self::from(vector!(angle, v.0[0], v.0[1], v.0[2]))
    }
    pub fn from_angle_vect(angle: f32, v: Vector3) -> Self {
        Self::from(vector!(angle.to_radians(), v.0[0], v.0[1], v.0[2]))
    }

    
    pub fn to_matrix4x4(&self) -> Matrix4x4 {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        Matrix::from_arrays([
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
                0.0
            ],
            [
                2.0*(x*z - y*w),
                2.0*(y*z + x*w),
                1.0 - 2.0*(x*x + y*y),
                0.0
            ],
            [
                0.0,
                0.0,
                0.0,
                1.0,
            ]
        ])
    }
    pub const IDENTITY: Self = Self{w: 1.0, x: 0.0, y: 0.0, z: 0.0};

}
//TODO write unit test for quat operations
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

#[cfg(test)]
mod test {
    use std::f32::consts::PI;
    use crate::math::{Vector4, Vector3, Vector, vectors::Quaternion};

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn rotate_vector() {
        let q = Quaternion::from(Vector4::new([
            PI/2.0,
            0.0,
            0.0,
            1.0
        ]));
        dbg!(&q);
        let v = Vector4::new([1.0, 0.0, 0.0, 1.0]);
        let w = Vector4::new([0.0, 1.0, 0.0, 1.0]);
        dbg!(v.rotate(q), w);
    }

    #[test]
    fn quat_ident_mul() {
        let q = Quaternion { w: 0.70710677, x: 0.0, y: 0.70710677, z: 0.0 };
        let id = Quaternion::IDENTITY;
        
        let mul = q*id;
        assert!(approx_eq(mul.w, q.w));
        assert!(approx_eq(mul.x, q.x));
        assert!(approx_eq(mul.y, q.y));
        assert!(approx_eq(mul.z, q.z));
    }
        #[test]
    fn quat_mul() {
        let q = Quaternion { w: 0.70710677, x: 0.0, y: 0.70710677, z: 0.0 };
        let id = Quaternion::from_angle_vect(90.0, vector!(0.0, 1.0, 0.0));
        
        let mul = q*id;
        dbg!(mul);
    }
}