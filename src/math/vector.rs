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

    //pub fn length(&self) -> f32
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