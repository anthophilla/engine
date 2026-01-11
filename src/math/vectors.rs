#[macro_export]
macro_rules! vector {
    ($x:expr, &y:expr) => {
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

// impl Vector4 {
//     fn scale(&self, x: f32, y: f32, z: f32, w: f32) -> Self {

//     }
// }