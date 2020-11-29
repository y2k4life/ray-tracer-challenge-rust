use crate::{float_eq, Point, Vector};
use std::{
    fmt,
    ops::{Index, IndexMut, Mul},
};

/// Matrix (plural matrices) is a rectangular array of numbers in rows and
/// columns that is treated as a single entity and manipulated according
/// to particular rules.
#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    data: [[f64; 4]; 4],
    inverse: [[f64; 4]; 4],
}

/// A matrix in which all the elements of the principal diagonal are ones
/// and all other elements are zeros. The effect of multiplying a given matrix
/// by an identity matrix is to leave the given matrix unchanged.
pub const IDENTITY: Matrix = Matrix {
    data: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
    inverse: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Matrix {
    /// Creates a Matrix with the provide 4x4 array of [`f64`] numbers. Even
    /// though the storage of an array is 4x4 the matrix is used for 3x3 and
    /// 2x2 matrices.
    ///
    /// Calculate the a matrix A-1 which is called the inverse of A such that:
    /// A * A-1 = A-1 * A = I, where I is the identity matrix. Multiply matrix
    /// A by matrix B, production C, then C can be multiplied by the inverse of
    /// B to get A. Similar to scalar numbers multiply A * B = C, to get A by
    /// inverting B and multiplying by C. For example 5 * 4 = 20 the inverse of
    /// B is 1/4 and 1/4 * 20 is 5.
    ///
    /// # Example
    ///
    /// ```
    /// use rustic_ray::Matrix;
    ///
    /// let m = Matrix::new([
    ///     [ 1.0,  2.0,  3.0,  4.0],
    ///     [ 5.5,  6.5,  7.5,  8.5],
    ///     [ 9.0, 10.0, 11.0, 12.0],
    ///     [13.5, 14.5, 15.5, 16.5],
    /// ]);
    ///
    /// assert_eq!(m[0][0], 1.0);
    /// assert_eq!(m[0][3], 4.0);
    /// assert_eq!(m[1][0], 5.5);
    /// assert_eq!(m[1][2], 7.5);
    /// assert_eq!(m[2][2], 11.0);
    /// assert_eq!(m[3][0], 13.5);
    /// assert_eq!(m[3][2], 15.5);
    /// ```
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        let mut inverse = [[0.0; 4]; 4];
        let d = Matrix::determinant(data, 4);
        for row in 0..4 {
            for col in 0..4 {
                inverse[col][row] = Matrix::cofactor(data, row, col, 3) / d;
            }
        }

        Self { data, inverse }
    }

    // Create a new matrix from the inverse data from `self`.
    pub fn inverse(&self) -> Matrix {
        Matrix {
            data: self.inverse,
            inverse: self.data,
        }
    }

    /// Switch the rows and column indices of a matrix, it flips a matrix over
    /// its diagonal. Used for translating normal vectors between object space
    /// and world space.
    ///
    /// # Example
    /// ```
    /// use rustic_ray::Matrix;
    ///
    /// let m1 = Matrix::new([
    ///     [0.0, 9.0, 3.0, 0.0],
    ///     [9.0, 8.0, 0.0, 8.0],
    ///     [1.0, 8.0, 5.0, 3.0],
    ///     [0.0, 0.0, 5.0, 8.0],
    /// ]);
    ///
    /// let expected = Matrix::new([
    ///     [0.0, 9.0, 1.0, 0.0],
    ///     [9.0, 8.0, 8.0, 0.0],
    ///     [3.0, 0.0, 5.0, 5.0],
    ///     [0.0, 8.0, 3.0, 8.0],
    /// ]);
    ///
    /// assert_eq!(m1.transpose(), expected);
    /// ```
    pub fn transpose(&self) -> Self {
        let d = [
            [
                self.data[0][0],
                self.data[1][0],
                self.data[2][0],
                self.data[3][0],
            ],
            [
                self.data[0][1],
                self.data[1][1],
                self.data[2][1],
                self.data[3][1],
            ],
            [
                self.data[0][2],
                self.data[1][2],
                self.data[2][2],
                self.data[3][2],
            ],
            [
                self.data[0][3],
                self.data[1][3],
                self.data[2][3],
                self.data[3][3],
            ],
        ];

        let it = [
            [
                self.inverse[0][0],
                self.inverse[1][0],
                self.inverse[2][0],
                self.inverse[3][0],
            ],
            [
                self.inverse[0][1],
                self.inverse[1][1],
                self.inverse[2][1],
                self.inverse[3][1],
            ],
            [
                self.inverse[0][2],
                self.inverse[1][2],
                self.inverse[2][2],
                self.inverse[3][2],
            ],
            [
                self.inverse[0][3],
                self.inverse[1][3],
                self.inverse[2][3],
                self.inverse[3][3],
            ],
        ];

        Matrix {
            data: d,
            inverse: it,
        }
    }

    /// Test if matrix `self` can be inverted
    pub fn is_invertible(&self) -> bool {
        !(Matrix::determinant(self.data, 4) == 0.0)
    }

    fn determinant(a: [[f64; 4]; 4], s: usize) -> f64 {
        let mut det = 0.;

        if s == 2 {
            det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
        } else {
            for col in 0..4 {
                det += a[0][col] * Matrix::cofactor(a, 0, col, s - 1);
            }
        }

        det
    }

    fn sub_matrix(a: [[f64; 4]; 4], r_row: usize, r_col: usize) -> [[f64; 4]; 4] {
        let mut m = [[0.0; 4]; 4];

        for (nri, ri) in [0, 1, 2, 3].iter().filter(|&&x| x != r_row).enumerate() {
            for (nci, ci) in [0, 1, 2, 3].iter().filter(|&&x| x != r_col).enumerate() {
                m[nri][nci] = a[*ri][*ci];
            }
        }

        m
    }

    fn minor(a: [[f64; 4]; 4], r_row: usize, r_col: usize, s: usize) -> f64 {
        Matrix::determinant(Matrix::sub_matrix(a, r_row, r_col), s)
    }

    fn cofactor(a: [[f64; 4]; 4], r_row: usize, r_col: usize, s: usize) -> f64 {
        let mut minor = Matrix::minor(a, r_row, r_col, s);
        if (r_row + r_col) % 2 == 1 {
            minor *= -1.0
        }
        minor
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Matrix) -> Self {
        let mut results = [[0.0; 4]; 4];

        for row in 0..4 {
            for col in 0..4 {
                results[row][col] = self[row][0] * other[0][col]
                    + self[row][1] * other[1][col]
                    + self[row][2] * other[2][col]
                    + self[row][3] * other[3][col];
            }
        }

        Matrix::new(results)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        let c1 = (self[0][0] * other.x)
            + (self[0][1] * other.y)
            + (self[0][2] * other.z)
            + (self[0][3] * 1.0);
        let c2 = (self[1][0] * other.x)
            + (self[1][1] * other.y)
            + (self[1][2] * other.z)
            + (self[1][3] * 1.0);
        let c3 = (self[2][0] * other.x)
            + (self[2][1] * other.y)
            + (self[2][2] * other.z)
            + (self[2][3] * 1.0);

        Point::new(c1, c2, c3)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        let c1 = (self[0][0] * other.x)
            + (self[0][1] * other.y)
            + (self[0][2] * other.z)
            + (self[0][3] * 0.0);
        let c2 = (self[1][0] * other.x)
            + (self[1][1] * other.y)
            + (self[1][2] * other.z)
            + (self[1][3] * 0.0);
        let c3 = (self[2][0] * other.x)
            + (self[2][1] * other.y)
            + (self[2][2] * other.z)
            + (self[2][3] * 0.0);

        Vector::new(c1, c2, c3)
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                if !float_eq(self[r][c], other[r][c]) {
                    return false;
                }
            }
        }

        true
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0:>10}", format!("{0:.5}", self.data[0][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[0][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[0][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[0][3]))?;

        write!(f, "{0:>10}", format!("{0:.5}", self.data[1][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[1][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[1][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[1][3]))?;

        write!(f, "{0:>10}", format!("{0:.5}", self.data[2][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[2][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[2][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[2][3]))?;

        write!(f, "{0:>10}", format!("{0:.5}", self.data[3][0]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[3][1]))?;
        write!(f, "{0:>10}", format!("{0:.5}", self.data[3][2]))?;
        writeln!(f, "{0:>10}", format!("{0:.5}", self.data[3][3]))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Matrix, IDENTITY};
    use crate::{float_eq, Point};

    // Chapter 3 Matrices
    // Page 26
    #[test]
    #[rustfmt::skip]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::new([
            [ 1.0,  2.0,  3.0,  4.0],
            [ 5.5,  6.5,  7.5,  8.5],
            [ 9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], 15.5);
    }

    // Chapter 3 Matrices
    // Page 27
    #[test]
    #[rustfmt::skip]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix::new([
            [-3.0,  5.0, 0.0, 0.0],
            [ 1.0, -2.0, 0.0, 0.0],
            [ 0.0,  0.0, 0.0, 0.0],
            [ 0.0,  0.0, 0.0, 0.0],
        ]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[0][1], 5.0);
        assert_eq!(m[1][0], 1.0);
        assert_eq!(m[1][1], -2.0);
    }

    // Chapter 3 Matrices
    // Page 27
    #[test]
    #[rustfmt::skip]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix::new([
            [-3.0,  5.0,  0.0, 0.0],
            [ 1.0, -2.0, -7.0, 0.0],
            [ 0.0,  1.0,  1.0, 0.0],
            [ 0.0,  0.0,  0.0, 0.0],
        ]);

        assert_eq!(m[0][0], -3.0);
        assert_eq!(m[1][1], -2.0);
        assert_eq!(m[2][2], 1.0);
    }

    // Chapter 3 Matrices
    // page 27
    #[test]
    fn matrix_equality_with_identical_matrices() {
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(m1, m2);
    }

    // Chapter 3 Matrices
    // Page 27 & 28
    #[test]
    fn matrix_equality_with_different_matrices() {
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(m1, m2);
    }

    // Chapter 3 Matrices
    // Page 28
    #[test]
    #[rustfmt::skip]
    fn multiple_two_matrices() {
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new([
            [-2.0, 1.0, 2.0,  3.0],
            [ 3.0, 2.0, 1.0, -1.0],
            [ 4.0, 3.0, 6.0,  5.0],
            [ 1.0, 2.0, 7.0,  8.0],
        ]);

        let expected = Matrix::new([
            [20.0, 22.0,  50.0,  48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0,  46.0,  42.0],
        ]);

        assert_eq!(m1 * m2, expected);
    }

    // Chapter 3 Matrices
    // Page 30
    #[test]
    fn a_matrix_multiplied_by_a_point_tuple() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let b = Point::new(1.0, 2.0, 3.0);

        assert_eq!(m * b, Point::new(18.0, 24.0, 33.0))
    }

    // Chapter 3 Matrices
    // Page 32
    #[test]
    #[rustfmt::skip]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let m1 = Matrix::new([
            [0.0, 1.0,  2.0,  4.0],
            [1.0, 2.0,  4.0,  8.0],
            [2.0, 4.0,  8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(IDENTITY * m1, m1);
    }

    // Chapter 3 Matrices
    // Page 33
    #[test]
    fn transposing_a_matrix() {
        let m1 = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let expected = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(m1.transpose(), expected);
    }

    // Chapter 3 Matrices
    // Page 33
    #[test]
    fn transpose_the_identity_matrix() {
        let a = IDENTITY.transpose();

        assert_eq!(a, IDENTITY);
    }

    // Chapter 3 Matrices
    // Page 34
    #[test]
    #[rustfmt::skip]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let m = [
            [ 1.0, 5.0, 0.0, 0.0],
            [-3.0, 2.0, 0.0, 0.0],
            [ 0.0, 0.0, 0.0, 0.0],
            [ 0.0, 0.0, 0.0, 0.0],
        ];

        assert_eq!(Matrix::determinant(m, 2), 17.0);
    }

    // Chapter 3 Matrices
    // Page 35
    #[test]
    #[rustfmt::skip]
    fn a_sub_matrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let m = [
            [ 1.0, 5.0,  0.0, 0.0],
            [-3.0, 2.0,  7.0, 0.0],
            [ 0.0, 6.0, -3.0, 0.0],
            [ 0.0, 0.0,  0.0, 0.0],
        ];
        let actual = Matrix::new(Matrix::sub_matrix(m, 0, 2));
        
        let expected = Matrix::new([
            [-3.0, 2.0, 0.0, 0.0],
            [ 0.0, 6.0, 0.0, 0.0],
            [ 0.0, 0.0, 0.0, 0.0],
            [ 0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(actual, expected);
    }

    // Chapter 3 Matrices
    // Page 35
    #[test]
    #[rustfmt::skip]
    fn a_sub_matrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let m = [
            [-6.0, 1.0,  1.0, 6.0],
            [-8.0, 5.0,  8.0, 6.0],
            [-1.0, 0.0,  8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ];
        let actual = Matrix::new(Matrix::sub_matrix(m, 2, 1));
        let expected = Matrix::new([
            [-6.0,  1.0, 6.0, 0.0],
            [-8.0,  8.0, 6.0, 0.0],
            [-7.0, -1.0, 1.0, 0.0],
            [ 0.0,  0.0, 0.0, 0.0],
        ]);

        assert_eq!(actual, expected);
    }
    // Chapter 3 Matrices
    // Page 35
    #[test]
    #[rustfmt::skip]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = [
            [3.0,  5.0,  0.0, 0.0],
            [2.0, -1.0, -7.0, 0.0],
            [6.0, -1.0,  5.0, 0.0],
            [0.0,  0.0,  0.0, 0.0],
        ];
        let b = Matrix::sub_matrix(a, 1, 0);
        
        assert_eq!(Matrix::determinant(b, 2), 25.0);
        assert_eq!(Matrix::minor(a, 1, 0, 2), 25.0);
    }

    // Chapter 3 Matrices
    // Page 36
    #[test]
    #[rustfmt::skip]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = [
            [3.0,  5.0,  0.0, 0.0],
            [2.0, -1.0, -7.0, 0.0],
            [6.0, -1.0,  5.0, 0.0],
            [0.0,  0.0,  0.0, 0.0],
        ];

        assert_eq!(Matrix::minor(a, 0, 0, 2), -12.0);
        assert_eq!(Matrix::cofactor(a, 0, 0, 2), -12.0);
        assert_eq!(Matrix::minor(a, 1, 0, 2), 25.0);
        assert_eq!(Matrix::cofactor(a, 1, 0, 2), -25.0);
    }

    // Chapter 3 Matrices
    // Page 37
    #[test]
    #[rustfmt::skip]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = [
            [ 1.0, 2.0,  6.0, 0.0],
            [-5.0, 8.0, -4.0, 0.0],
            [ 2.0, 6.0,  4.0, 0.0],
            [ 0.0, 0.0,  0.0, 0.0],
        ];

        assert_eq!(Matrix::cofactor(a, 0, 0, 2), 56.0);
        assert_eq!(Matrix::cofactor(a, 0, 1, 2), 12.0);
        assert_eq!(Matrix::cofactor(a, 0, 2, 2), -46.0);
        assert_eq!(Matrix::determinant(a, 3), -196.0);
    }

    // Chapter 3 Matrices
    // Page 37
    #[test]
    #[rustfmt::skip]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = [
            [-2.0, -8.0,  3.0,  5.0],
            [-3.0,  1.0,  7.0,  3.0],
            [ 1.0,  2.0, -9.0,  6.0],
            [-6.0,  7.0,  7.0, -9.0],
        ];

        assert_eq!(Matrix::cofactor(a, 0, 0, 3), 690.0);
        assert_eq!(Matrix::cofactor(a, 0, 1, 3), 447.0);
        assert_eq!(Matrix::cofactor(a, 0, 2, 3), 210.0);
        assert_eq!(Matrix::cofactor(a, 0, 3, 3), 51.0);
        assert_eq!(-4071.0, Matrix::determinant(a, 4));
    }

    // Chapter 3 Matrices
    // Page 39
    #[test]
    #[rustfmt::skip]
    fn testing_an_invertible_matrix_for_invartibility() {
        let a = Matrix::new([
            [6.0,  4.0, 4.0,  4.0],
            [5.0,  5.0, 7.0,  6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0,  1.0, 7.0, -6.0],
        ]);

        assert_eq!(Matrix::determinant(a.data, 4), -2120.0);
        assert!(a.is_invertible());
    }

    // Chapter 3 Matrices
    // Page 39
    #[test]
    #[rustfmt::skip]
    fn testing_an_none_invertible_matrix_for_invartibility() {
        let a = Matrix::new([
            [-4.0,  2.0, -2.0, -3.0],
            [ 9.0,  6.0,  2.0,  6.0],
            [ 0.0, -5.0,  1.0, -5.0],
            [ 0.0,  0.0,  0.0,  0.0],
        ]);

        assert_eq!(Matrix::determinant(a.data, 4), 0.0);
        assert_eq!(false, a.is_invertible())
    }

    // Chapter 3 Matrices
    // Page 39
    #[test]
    #[rustfmt::skip]
    fn calculating_the_inverse_of_a_matrix() {
        let m = [
            [-5.0,  2.0,  6.0, -8.0],
            [ 1.0, -5.0,  1.0,  8.0],
            [ 7.0,  7.0, -6.0, -7.0],
            [ 1.0, -3.0,  7.0,  4.0],
        ];
        let a = Matrix::new(m);
        let b = a.inverse();

        assert_eq!(532.0, Matrix::determinant(m, 4));
        assert_eq!(-160.0 / 532.0, b[3][2]);
        let expected = Matrix::new([
            [ 0.21805,  0.45113,  0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361,  0.52068],
            [-0.07895, -0.22368, -0.05263,  0.19737],
            [-0.52256, -0.81391, -0.30075,  0.30639],
        ]);
        for row in 0..4 {
            for col in 0..4 {
                let are_equal = float_eq(b.data[row][col], expected.data[row][col]);
                assert_eq!(true, are_equal);
            }
        }
    }

    // Chapter 3 Matrices
    // Page 41
    #[test]
    #[rustfmt::skip]
    fn calculating_the_inverse_of_another_matrix() {
        let m = Matrix::new([
            [ 8.0, -5.0,  9.0,  2.0],
            [ 7.0,  5.0,  6.0,  1.0],
            [-6.0,  0.0,  9.0,  6.0],
            [-3.0,  0.0, -9.0, -4.0],
        ]);

        let expected = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692,  0.12308,  0.02564,  0.03077],
            [ 0.35897,  0.35897,  0.43590,  0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(m.inverse(), expected);
    }

    // Chapter 3 Matrices
    // Page 41
    #[test]
    #[rustfmt::skip]
    fn calculating_the_inverse_of_a_third_matrix() {
        let m = Matrix::new([
            [ 9.0,  3.0,  0.0,  9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0,  9.0,  6.0,  4.0],
            [-7.0,  6.0,  6.0,  2.0],
        ]);

        let expected = Matrix::new([
            [-0.04074, -0.07778,  0.14444, -0.22222],
            [-0.07778,  0.03333,  0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926,  0.12963],
            [ 0.17778,  0.06667, -0.26667,  0.33333],
        ]);

        assert_eq!(m.inverse(), expected);
    }

    // Chapter 3 Matrices
    // Page 41
    #[test]
    #[rustfmt::skip]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix::new([
            [ 3.0, -9.0,  7.0,  3.0],
            [ 3.0, -8.0,  2.0, -9.0],
            [-4.0,  4.0,  4.0,  1.0],
            [-6.0,  5.0, -1.0,  1.0],
        ]);
        let b = Matrix::new([
            [8.0,  2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0,  0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let c = a * b;
        
        assert_eq!(c * b.inverse(), a);
    }
}
