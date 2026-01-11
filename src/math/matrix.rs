// macro_rules! matrix {
//     ($( $x:expr ), *) => {
        
//     };
// }

use crate::math::Vector;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Matrix<const X: usize, const Y: usize>([[f32; X]; Y]);
impl<const X: usize, const Y: usize> Matrix<X, Y> {
    pub fn from_arrays(arr: [[f32; X]; Y]) -> Self { Self(arr) }
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
impl<const X: usize, const Y: usize> std::ops::Mul<Vector<X>> for Matrix<X, Y> {
    type Output = Vector<Y>;
    fn mul(self, other: Vector<X>) -> Vector<Y> {
        Vector::new(
            std::array::from_fn(|y| {
                (0..X)
                    .map(|x| self.0[y][x] * other.0[x])
                    .sum()
            })
        )
    }
}

#[cfg(test)]
mod test {
    use crate::math::{Matrix, Vector3};

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
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ]);
        assert_eq!(a*Vector3::new([1.0, 2.0, 3.0]), Vector3::new([1.0, 2.0, 3.0]))
    }
}