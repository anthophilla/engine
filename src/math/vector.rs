use crate::math::{Mat3, Mat4, Quaternion};

#[macro_export]
macro_rules! vector {
    ($x: expr, $y: expr) => {
        Vector::<2>::new([$x, $y])
    };
    ($x: expr, $y: expr, $z: expr) => {
        Vector::<3>::new([$x, $y, $z])
    };
    ($x: expr, $y: expr, $z: expr, $w: expr) => {
        Vector::<4>::new([$x, $y, $z, $w])
    };
}

pub type Vector3 = Vector<3>;
pub type Vector4 = Vector<4>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector<const N: usize>(pub [f32; N]);
impl<const N: usize> Vector<N> {
    pub const fn new(v: [f32; N]) -> Self { Self(v) }
    pub const fn as_array(&self) -> [f32; N] { self.0 }

    /// length of the vector ( square root of the sum of the square of every component) [sqrt(x*x + y*y + ..)]
    pub fn length(&self) -> f32 {
        //will implement Iter for Vector later
        self.0.iter().map(|x| x*x).sum::<f32>().sqrt()
    }
    
    /// all components divided by length
    pub fn normalized(&self) -> Self {
        let length = self.length();
        Self::new(std::array::from_fn(|x| self[x]/length))
    }

    /// ax*bx + ay*by + ...
    pub fn dot(&self, other: &Self) -> f32 {
        (0..N).map(|i| self[i]*other[i]).sum()
    }
}
impl<const N: usize> std::ops::Index<usize> for Vector<N> {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
//TODO
// impl<const N: usize> std::fmt::Debug for Vector<N> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple(format!("vector<{N}>")).field(value)
//     }
// }
impl<const N: usize> Into<[f32; N]> for Vector<N> {
    fn into(self) -> [f32; N] {
        self.0
    }
}


// scalar operations
impl<const N: usize> std::ops::Add for Vector<N> {
    type Output = Self;
    ///add every component together
    fn add(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self[i] + other[i]))
    }
}
impl<const N: usize> std::ops::Sub for Vector<N> {
    type Output = Self;
    ///subtract every component together
    fn sub(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self[i] - other[i]))
    }
}
impl<const N: usize> std::ops::Mul for Vector<N> {
    type Output = Self;
    ///multiply every component together
    fn mul(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self[i] * other[i]))
    }
}
impl<const N: usize> std::ops::Div for Vector<N> {
    type Output = Self;
    ///divide every component together
    fn div(self, other: Self) -> Self {
        Self(std::array::from_fn(|i| self[i] / other[i]))
    }
}


impl<const N: usize> std::ops::Mul<f32> for Vector<N> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self(std::array::from_fn(|i| self[i]*rhs))
    }
}

impl Vector4 {
    pub fn scaled(self, scale: Vector3) -> Self {
        Mat4::from_arrays([
            [scale[0], 0.0, 0.0, 0.0],
            [0.0, scale[1], 0.0, 0.0],
            [0.0, 0.0, scale[2], 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ])*self
    }

    pub fn translated(self, trans: Vector3) -> Self {
        Mat4::from_arrays([
            [0.0, 0.0, 0.0, trans[0]],
            [0.0, 0.0, 0.0, trans[1]],
            [0.0, 0.0, 0.0, trans[2]],
            [0.0, 0.0, 0.0, 1.0]
        ])*self
    }

    pub fn rotated(self, rot: Quaternion) -> Self {
        Mat4::from(rot) * self
    }
}

impl Vector3 {
    pub fn scaled(self, scale: Vector3) -> Self {
        Mat3::from_arrays([
            [scale[0], 0.0, 0.0],
            [0.0, scale[1], 0.0],
            [0.0, 0.0, scale[2]],
        ])*self
    }
    pub fn translated(self, trans: Vector3) -> Self {
        Mat3::from_arrays([
            [0.0, 0.0, trans[0]],
            [0.0, 0.0, trans[1]],
            [0.0, 0.0, trans[2]],
        ])*self
    }
    pub fn rotated(self, rot: Quaternion) -> Self {
        Mat3::from(rot) * self
    }
}