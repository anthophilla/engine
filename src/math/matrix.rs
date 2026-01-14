// macro_rules! matrix {
//     ($( $x:expr ), *) => {
        
//     };
// }

use crate::math::Vector;

pub type Matrix3x3 = Matrix<3, 3>;
pub type Matrix4x4 = Matrix<4, 4>;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Matrix<const X: usize, const Y: usize>([[f32; X]; Y]);
impl<const X: usize, const Y: usize> Matrix<X, Y> {
    pub const fn from_arrays(arr: [[f32; X]; Y]) -> Self { Self(arr) }
    pub fn column_major(&self) -> [[f32; Y]; X] {
        let mut new = [[0.0; Y]; X];
        for y in 0..Y {
            for x in 0..X {
                new[x][y] = self.0[y][x];
            }
        };
        new
    }
}
impl<const X: usize, const Y: usize> std::ops::Add for Matrix<X, Y> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::from_arrays(
            std::array::from_fn(|y| 
                std::array::from_fn(|x| self.0[y][x] + other.0[y][x])
        )
        )
    }
}
impl<const X: usize, const Y: usize> std::ops::Sub for Matrix<X, Y> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::from_arrays(
            std::array::from_fn(|y| 
                std::array::from_fn(|x| self.0[y][x] - other.0[y][x])
        )
        )
    }
}
impl<const X: usize, const Y: usize> std::ops::Mul<f32> for Matrix<X, Y> {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self::from_arrays(
            std::array::from_fn(|y| 
                std::array::from_fn(|x| self.0[y][x] * other)
        )
        )
    }
}
impl<const X: usize, const Y: usize> std::ops::Mul<Matrix<Y, X>> for Matrix<X, Y> {
    type Output = Self;
    fn mul(self, other: Matrix<Y, X>) -> Self {
        Matrix::from_arrays(std::array::from_fn(|y|
            std::array::from_fn(|x|
                self.0[y][x] * other.0[x][y]
            )
        ))
    }
}
impl<const X: usize> std::ops::Mul<Vector<X>> for Matrix<X, X> {
    type Output = Vector<X>;
    fn mul(self, other: Vector<X>) -> Vector<X> {
        Vector::new(
            std::array::from_fn(|y| {
                (0..X)
                    .map(|x| {self.0[y][x] * other.0[x]})
                    .sum()
            })
        )
    }
}

impl Matrix4x4 {
    pub const IDENTITY: Self = Self::from_arrays([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
    pub fn as_gl_slice(&self) -> [f32; 16] {
        let m = self.column_major();
        return [
            m[0][0], m[0][1], m[0][2], m[0][3],
            m[1][0], m[1][1], m[1][2], m[1][3],
            m[2][0], m[2][1], m[2][2], m[2][3],
            m[3][0], m[3][1], m[3][2], m[3][2],
        ]
    }
}
impl Matrix3x3 {
    pub const IDENTITY: Self = Self::from_arrays([
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
    ]);
    // pub fn as_gl_slice(&self) -> [f32; 9] {
    //     let m = self.column_major();
    //     return [
    //         m[0][0], m[0][1], m[0][2],
    //         m[1][0], m[1][1], m[1][2],
    //         m[2][0], m[2][1], m[2][2],
    //     ]
    // }
}

#[cfg(test)]
mod test {
    use crate::math::{Matrix, Vector3, Vector4, matrix::Matrix4x4};

    #[test]
    fn matrix_ops() {
        let a = Matrix::from_arrays([
            [0.0, 1.0, 2.0],
            [3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0],
        ]);
        let b = Matrix::from_arrays([
            [8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0],
            [2.0, 1.0, 0.0],
        ]);

        assert_eq!(
            a+b,
            Matrix::from_arrays([[8.0;3];3])
        );
        assert_eq!(
            a-b,
            Matrix::from_arrays([
                [-8.0, -6.0, -4.0],
                [-2.0, 0.0, 2.0],
                [4.0, 6.0, 8.0]
            ])
        );
        assert_eq!(
            a*3.0,
            Matrix::from_arrays([
                [0.0, 3.0, 6.0],
                [9.0, 12.0, 15.0],
                [18.0, 21.0, 24.0],
            ])
        )

    }
    #[test]
    fn matrix_mul() {
        let a = Matrix::from_arrays([
            [0.0, 1.0, 2.0],
            [3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0],
        ]);
        let b = Matrix::from_arrays([
            [8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0],
            [2.0, 1.0, 0.0],
        ]);

        let r = Matrix::from_arrays([
            [0.0, 5.0, 4.0],
            [21.0, 16.0, 5.0],
            [36.0, 21.0, 0.0]
        ]);

        assert_eq!(a*b, r);
    }
    #[test]
    fn mat_vec_mul() {
        let a = Matrix::from_arrays([
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ]);
        assert_eq!(Matrix4x4::IDENTITY*Vector4::new([1.0, 2.0, 3.0, 4.0]), Vector4::new([1.0, 2.0, 3.0, 4.0]));
        assert_eq!(a*Vector3::new([1.0, 2.0, 3.0]), Vector3::new([14.0, 32.0, 50.0]))
    }
    #[test]
    fn column_major() {
        let b = Matrix::from_arrays([
            [8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0],
            [2.0, 1.0, 0.0],
        ]);
        dbg!(&b);
        dbg!(&b.column_major());
    }
}