use crate::math::{Vector3};

pub struct Matrix<const X: usize, const Y: usize>([[f32; X]; Y]);

pub type Mat3 = Matrix<3, 3>;
pub type Mat4 = Matrix<4, 4>;

impl<const X: usize, const Y: usize> Matrix<X, Y> {
    pub const fn from_arrays(arr: [[f32; X]; Y]) -> Self { Self(arr) }

    pub fn as_column_major(&self) -> [[f32; Y]; X] {
        let mut new = [[0.0; Y]; X];
        for y in 0..Y {
            for x in 0..X {
                new[x][y] = self[y][x]
            };
        };
        new
    }
}
impl<const X: usize, const Y: usize> std::ops::Index<usize> for Matrix<X, Y> {
    type Output = [f32; X];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Mat4 {
    pub const IDENTITY: Self = Self::from_arrays([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    pub fn translation_mat(v: Vector3) -> Self {
        Self::from_arrays([
            [1.0, 0.0, 0.0, v[0]],
            [0.0, 1.0, 0.0, v[1]],
            [0.0, 0.0, 1.0, v[2]],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }
}

impl Mat3 {
    pub const IDENTITY: Self = Self::from_arrays([
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]);
}