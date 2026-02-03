use crate::math::{Vector, Vector3};

#[derive(PartialEq, Clone, Copy, Debug)]
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

impl<const X: usize, const Y: usize> std::ops::Add for Matrix<X, Y> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_arrays(
            std::array::from_fn(
                |y| std::array::from_fn(|x| self[y][x]+rhs[y][x])
            )
        )
    }
}
impl<const X: usize, const Y: usize> std::ops::Sub for Matrix<X, Y> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_arrays(
            std::array::from_fn(
                |y| std::array::from_fn(|x| self[y][x]-rhs[y][x])
            )
        )
    }
}
impl<const X: usize, const Y: usize> std::ops::Mul<f32> for Matrix<X, Y> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_arrays(
            std::array::from_fn(
                |y| std::array::from_fn(|x| self[y][x]*rhs)
            )
        )
    }
}

impl<const X: usize, const Y: usize> std::ops::Mul<Matrix<Y, X>> for Matrix<X, Y> {
    type Output = Self;

    fn mul(self, rhs: Matrix<Y, X>) -> Self::Output {
        Matrix::from_arrays(
            std::array::from_fn(
                |y| std::array::from_fn(|x| self[y][x]*rhs[x][y])
            )
        )
    }
}
impl<const X: usize> std::ops::Mul<Vector<X>> for Matrix<X, X> {
    type Output = Vector<X>;
    fn mul(self, rhs: Vector<X>) -> Self::Output {
        Vector::new(
            std::array::from_fn(|y| {
                (0..X).map(|x| {self[y][x]*rhs[x]}).sum()
            })
        )
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

//some stupid tests ill never use (maybe ill delete them later) (copied from the previous version)
#[cfg(test)]
mod test {
    use crate::math::{Matrix, Vector3, Vector4, matrix::Mat4};

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
        assert_eq!(Mat4::IDENTITY*Vector4::new([1.0, 2.0, 3.0, 4.0]), Vector4::new([1.0, 2.0, 3.0, 4.0]));
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
        dbg!(&b.as_column_major());
    }
}