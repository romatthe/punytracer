use crate::core::float::ApproxEq;
use crate::core::tuple::Tuple;

pub type Matrix2 = Matrix<2>;
pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

#[derive(Clone, Copy, Debug)]
struct Matrix<const N: usize> {
    data: [[f64; N]; N]
}

impl<const N: usize> Matrix<N> {
    pub fn new() -> Self {
        Self { data: [[0.0; N]; N], }
    }

    pub fn from(data: [[f64; N]; N]) -> Self {
        Self { data }
    }

    pub fn transpose(&self) -> Self {
        let mut transposed = Self::new();

        for x in 0..N {
            for y in 0..N {
                transposed[(x, y)] = self[(y, x)];
            }
        }

        transposed
    }
}

impl Matrix<2> {
    pub fn identity() -> Self {
        Self {
            data: [[1.0, 0.0],
                   [0.0, 1.0]],
        }
    }

    pub fn determinant(&self) -> f64 {
        self[(0,0)] * self[(1,1)] - self[(0,1)] * self[(1,0)]
    }
}

impl Matrix<3> {
    pub fn identity() -> Self {
        Self {
            data: [[1.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0],
                   [0.0, 0.0, 1.0]],
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<2> {
        let mut sub = Matrix::<2>::new();
        let mut new_row = 0;
        for x in 0..3{
            let mut new_col = 0;
            if x != row {
                for y in 0..3 {
                    if y != col {
                        sub[(new_row,new_col)] = self[(x,y)];
                        new_col += 1;
                    }
                }
                new_row += 1;
            }
        }

        sub
    }

    pub fn determinant(&self) -> f64 {
        self[(0,0)] * self.cofactor(0, 0)
            + self[(0,1)] * self.cofactor(0, 1)
            + self[(0,2)] * self.cofactor(0, 2)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        match row + col {
            n if n % 2 == 0 => self.minor(row, col),
            _               => -self.minor(row, col),
        }
    }
}

impl Matrix<4> {
    pub fn identity() -> Self {
        Self {
            data: [[1.0, 0.0, 0.0, 0.0],
                   [0.0, 1.0, 0.0, 0.0],
                   [0.0, 0.0, 1.0, 0.0],
                   [0.0, 0.0, 0.0, 1.0]],
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix<3> {
        let mut sub = Matrix::<3>::new();
        let mut new_row = 0;
        for x in 0..4 {
            let mut new_col = 0;
            if x != row {
                for y in 0..4 {
                    if y != col {
                        sub[(new_row,new_col)] = self[(x,y)];
                        new_col += 1;
                    }
                }
                new_row += 1;
            }
        }

        sub
    }

    pub fn determinant(&self) -> f64 {
        self[(0,0)] * self.cofactor(0, 0)
            + self[(0,1)] * self.cofactor(0, 1)
            + self[(0,2)] * self.cofactor(0, 2)
            + self[(0,3)] * self.cofactor(0, 3)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        match row + col {
            n if n % 2 == 0 => self.minor(row, col),
            _               => -self.minor(row, col),
        }
    }

    pub fn is_invertible(&self) -> bool {
        !self.determinant().approx_eq(0.0)
    }

    pub fn invert(&self) -> Self {
        if !self.is_invertible() {
            panic!("Could not invert matrix {:?}", self);
        }

        let mut m2 = Self::new();

        for row in 0..4 {
            for col in 0..4 {
                m2[(col,row)] = self.cofactor(row, col) / self.determinant();
            }
        }

        m2
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Self) -> bool {
        for x in 0..N {
            for y in 0..N {
                if !self[(x,y)].approx_eq_low_precision(other[(x,y)]) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const N: usize> std::ops::Index<(usize, usize)> for Matrix<N> {
    type Output = f64;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x][y]
    }
}

impl<const N: usize> std::ops::IndexMut<(usize, usize)> for Matrix<N> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x][y]
    }
}

impl<const N: usize> std::ops::Mul for Matrix<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = Self::new();

        for row in 0..N {
            for col in 0..N {
                res[(row,col)] = (0..N)
                    .map(|n| self[(row, n)] * rhs[(n, col)])
                    .reduce(|a, b| a + b).unwrap();
            }
        }

        res
    }
}

impl<T> std::ops::Mul<T> for Matrix<4>
    where T: Tuple
{
    type Output = T;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output::new(
            self[(0,0)] * rhs.x() + self[(0,1)] * rhs.y() + self[(0,2)] * rhs.z() + self[(0,3)] * rhs.w(),
            self[(1,0)] * rhs.x() + self[(1,1)] * rhs.y() + self[(1,2)] * rhs.z() + self[(1,3)] * rhs.w(),
            self[(2,0)] * rhs.x() + self[(2,1)] * rhs.y() + self[(2,2)] * rhs.z() + self[(2,3)] * rhs.w(),
        )
    }
}

mod tests {
    use crate::core::point::Point;
    use super::*;

    #[test]
    fn create_matrix_2() {
        // Given
        let mut matrix = Matrix2::new();

        matrix[(0,0)] = -3.0;
        matrix[(0,1)] = 5.0;
        matrix[(1,0)] = 1.0;
        matrix[(1,1)] = -2.0;

        // Then
        assert_eq!(matrix[(0,0)], -3.0);
        assert_eq!(matrix[(0,1)], 5.0);
        assert_eq!(matrix[(1,0)], 1.0);
        assert_eq!(matrix[(1,1)], -2.0);
    }

    #[test]
    fn create_matrix_3() {
        // Given
        let mut matrix = Matrix3::new();

        matrix[(0,0)] = -3.0;
        matrix[(0,1)] = 5.0;
        matrix[(1,0)] = 1.0;
        matrix[(1,1)] = -2.0;
        matrix[(1,2)] = -7.0;
        matrix[(2,1)] = 1.0;
        matrix[(2,2)] = 1.0;

        // Then
        assert_eq!(matrix[(0,0)], -3.0);
        assert_eq!(matrix[(0,1)], 5.0);
        assert_eq!(matrix[(1,0)], 1.0);
        assert_eq!(matrix[(1,1)], -2.0);
        assert_eq!(matrix[(1,2)], -7.0);
        assert_eq!(matrix[(2,1)], 1.0);
        assert_eq!(matrix[(2,2)], 1.0);
    }

    #[test]
    fn create_matrix_4() {
        // Given
        let mut matrix = Matrix4::new();

        matrix[(0,0)] = 1.0;
        matrix[(0,3)] = 4.0;
        matrix[(1,0)] = 5.5;
        matrix[(1,2)] = 7.5;
        matrix[(2,2)] = 11.0;
        matrix[(3,0)] = 13.5;
        matrix[(3,2)] = 15.5;

        // Then
        assert_eq!(matrix[(0,0)], 1.0);
        assert_eq!(matrix[(0,3)], 4.0);
        assert_eq!(matrix[(1,0)], 5.5);
        assert_eq!(matrix[(1,2)], 7.5);
        assert_eq!(matrix[(2,2)], 11.0);
        assert_eq!(matrix[(3,0)], 13.5);
        assert_eq!(matrix[(3,2)], 15.5);
    }

    #[test]
    fn matrix_equality_with_same_matrix() {
        // Given
        let m1 = Matrix4::from(
            [[ 1.0, 2.0, 3.0, 4.0 ],
             [ 5.0, 6.0, 7.0, 8.0 ],
             [ 9.0, 8.0, 7.0, 6.0 ],
             [ 5.0, 4.0, 3.0, 2.0 ]]
        );
        let m2 = Matrix4::from(
            [[ 1.0, 2.0, 3.0, 4.0 ],
             [ 5.0, 6.0, 7.0, 8.0 ],
             [ 9.0, 8.0, 7.0, 6.0 ],
             [ 5.0, 4.0, 3.0, 2.0 ]]
        );

        // Then
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_equality_with_different_matrix() {
        // Given
        let m1 = Matrix4::from(
            [[ 1.0, 2.0, 3.0, 4.0 ],
             [ 5.0, 6.0, 7.0, 8.0 ],
             [ 9.0, 8.0, 7.0, 6.0 ],
             [ 5.0, 4.0, 3.0, 2.0 ]]
        );
        let m2 = Matrix4::from(
            [[ 1.0, 2.0, 3.0, 5.0 ],
             [ 5.0, 6.0, 7.0, 8.0 ],
             [ 9.0, 8.0, 7.0, 6.0 ],
             [ 5.0, 4.0, 3.0, 2.0 ]]
        );

        // Then
        assert_ne!(m1, m2);
    }

    #[test]
    fn matrix_multiplication() {
        // Given
        let m1 = Matrix4::from(
            [[ 1.0, 2.0, 3.0, 4.0 ],
             [ 5.0, 6.0, 7.0, 8.0 ],
             [ 9.0, 8.0, 7.0, 6.0 ],
             [ 5.0, 4.0, 3.0, 2.0 ]]
        );
        let m2 = Matrix4::from(
            [[ -2.0, 1.0, 2.0, 3.0 ],
             [ 3.0, 2.0, 1.0, -1.0 ],
             [ 4.0, 3.0, 6.0, 5.0 ],
             [ 1.0, 2.0, 7.0, 8.0 ]]
        );

        // Then
        let res = Matrix4::from(
            [[ 20.0, 22.0, 50.0, 48.0 ],
             [ 44.0, 54.0, 114.0, 108.0 ],
             [ 40.0, 58.0, 110.0, 102.0 ],
             [ 16.0, 26.0, 46.0, 42.0 ]]
        );

        assert_eq!(m1 * m2, res);
    }

    #[test]
    fn matrix_multiplied_by_a_tuple() {
        // Given
        let m = Matrix4::from(
            [[ 1.0, 2.0, 3.0, 4.0 ],
             [ 2.0, 4.0, 4.0, 2.0 ],
             [ 8.0, 6.0, 4.0, 1.0 ],
             [ 0.0, 0.0, 0.0, 1.0 ]]
        );
        let t = Point::new(1.0, 2.0, 3.0);

        // Then
        assert_eq!(m * t, Point::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn matrix_multiplied_by_identity() {
        // Given
        let m = Matrix4::from(
            [[ 1.0, 2.0, 3.0, 4.0 ],
             [ 2.0, 4.0, 4.0, 2.0 ],
             [ 8.0, 6.0, 4.0, 1.0 ],
             [ 0.0, 0.0, 0.0, 1.0 ]]
        );

        // Then
        assert_eq!(m * Matrix4::identity(), m);
    }

    #[test]
    fn identity_matrix_multiplied_by_tuple() {
        // Given
        let p = Point::new(1.0, 5.0, 3.0);

        // Then
        assert_eq!(Matrix4::identity() * p, p);
    }

    #[test]
    fn transposing_a_matrix() {
        // Given
        let matrix = Matrix4::from(
            [[ 0.0, 9.0, 3.0, 0.0 ],
             [ 9.0, 8.0, 0.0, 8.0 ],
             [ 1.0, 8.0, 5.0, 3.0 ],
             [ 0.0, 0.0, 5.0, 8.0 ]]
        );
        let transposed = Matrix4::from(
            [[ 0.0, 9.0, 1.0, 0.0 ],
             [ 9.0, 8.0, 8.0, 0.0 ],
             [ 3.0, 0.0, 5.0, 5.0 ],
             [ 0.0, 8.0, 3.0, 8.0 ]]
        );

        // Then
        assert_eq!(matrix.transpose(), transposed);
    }

    #[test]
    fn transposing_identity_matrix() {
        assert_eq!(Matrix4::identity().transpose(), Matrix4::identity());
    }

    #[test]
    fn determinant_of_2x2_matrix() {
        // Given
        let m = Matrix2::from([[1.0, 5.0], [-3.0, 2.0]]);

        // Then
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix() {
        // Given
        let matrix = Matrix3::from(
            [[ 1.0, 5.0, 0.0 ],
             [ -3.0, 2.0, 7.0 ],
             [ 0.0, 6.0, -3.0 ]]
        );
        let sub = Matrix2::from([[-3.0, 2.0], [0.0, 6.0]]);

        assert_eq!(matrix.submatrix(0, 2), sub);
    }

    #[test]
    fn submatrix_of_4x4_matrix() {
        // Given
        let matrix = Matrix4::from(
            [[ -6.0, 1.0, 1.0, 6.0 ],
             [ -8.0, 5.0, 8.0, 6.0 ],
             [ -1.0, 0.0, 8.0, 2.0 ],
             [ -7.0, 1.0, -1.0, 1.0 ]]
        );
        let sub = Matrix3::from(
            [[ -6.0, 1.0, 6.0 ],
             [ -8.0, 8.0, 6.0 ],
             [ -7.0, -1.0, 1.0 ]]
        );

        // Then
        assert_eq!(matrix.submatrix(2, 1), sub);
    }

    #[test]
    fn minor_of_3x3_matrix() {
        // Given
        let m = Matrix3::from(
            [[ 3.0, 5.0, 0.0 ],
             [ 2.0, -1.0, -7.0 ],
             [ 6.0, -1.0, 5.0 ]]
        );
        let sub = m.submatrix(1, 0);

        assert_eq!(sub.determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        // Given
        let m = Matrix3::from(
            [[ 3.0, 5.0, 0.0 ],
             [ 2.0, -1.0, -7.0 ],
             [ 6.0, -1.0, 5.0 ]]
        );

        // Then
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_determinant_of_3x3_matrix() {
        // Given
        let m = Matrix3::from(
            [[ 1.0, 2.0, 6.0 ],
             [ -5.0, 8.0, -4.0 ],
             [ 2.0, 6.0, 4.0 ]]
        );

        // Then
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn calculating_determinant_of_4x4_matrix() {
        // Given
        let m = Matrix4::from(
            [[ -2.0, -8.0, 3.0, 5.0 ],
             [ -3.0, 1.0, 7.0, 3.0 ],
             [ 1.0, 2.0, -9.0, 6.0 ],
             [ -6.0, 7.0, 7.0, -9.0 ]]
        );

        // Then
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn invertible_matrix_can_invert() {
        // Given
        let m = Matrix4::from(
            [[ 6.0, 4.0, 4.0, 4.0 ],
             [ 5.0, 5.0, 7.0, 6.0 ],
             [ 4.0, -9.0, 3.0, -7.0 ],
             [ 9.0, 1.0, 7.0, -6.0 ]]
        );

        // Then
        assert_eq!(m.determinant(), -2120.0);
        assert_eq!(m.is_invertible(), true);
    }

    #[test]
    fn invertible_matrix_cannot_invert() {
        // Given
        let m = Matrix4::from(
            [[ -4.0, 2.0, -2.0, -3.0 ],
                [ 9.0, 6.0, 2.0, 6.0 ],
                [ 0.0, -5.0, 1.0, -5.0 ],
                [ 0.0, 0.0, 0.0, -0.0 ]]
        );

        // Then
        assert_eq!(m.determinant(), 0.0);
        assert_eq!(m.is_invertible(), false);
    }

    #[test]
    fn inverse_of_a_matrix() {
        // Given
        let m1 = Matrix4::from(
            [[ -5.0, 2.0, 6.0, -8.0 ],
             [ 1.0, -5.0, 1.0, 8.0 ],
             [ 7.0, 7.0, -6.0, -7.0 ],
             [ 1.0, -3.0, 7.0, 4.0 ]]
        );
        let m2 = m1.invert();

        // Then
        assert!(m2[(3,2)].approx_eq(-160.0 / 532.0));
        assert_eq!(m1.determinant(), 532.0);
        assert_eq!(m1.cofactor(2, 3), -160.0);
        assert_eq!(m2, Matrix4::from(
            [[ 0.21805, 0.45113, 0.24060, -0.04511 ],
             [ -0.80827, -1.45677, -0.44361, 0.52068 ],
             [ -0.07895, -0.22368, -0.05263, 0.19737 ],
             [ -0.52256, -0.81391, -0.30075, 0.30639 ]]
        ));
    }

    #[test]
    fn inverse_of_another_matrix() {
        // Given
        let matrix = Matrix4::from(
            [[ 8.0, -5.0, 9.0, 2.0 ],
             [ 7.0, 5.0, 6.0, 1.0 ],
             [ -6.0, 0.0, 9.0, 6.0 ],
             [ -3.0, 0.0, -9.0, -4.0 ]]
        );

        // Then
        assert_eq!(matrix.invert(), Matrix4::from(
            [[ -0.15385, -0.15385, -0.28205, -0.53846 ],
             [ -0.07692, 0.12308, 0.02564, 0.03077 ],
             [ 0.35897, 0.35897, 0.43590, 0.92308 ],
             [ -0.69231, -0.69231, -0.76923, -1.92308 ]]
        ));
    }

    #[test]
    fn inverse_of_a_third_matrix() {
        // Given
        let matrix = Matrix4::from(
            [[ 9.0, 3.0, 0.0, 9.0 ],
             [ -5.0, -2.0, -6.0, -3.0 ],
             [ -4.0, 9.0, 6.0, 4.0 ],
             [ -7.0, 6.0, 6.0, 2.0 ]]
        );

        // Then
        assert_eq!(matrix.invert(), Matrix4::from(
            [[ -0.04074, -0.07778, 0.14444, -0.22222 ],
             [ -0.07778, 0.03333, 0.36667, -0.33333 ],
             [ -0.02901, -0.14630, -0.10926, 0.12963 ],
             [ 0.17778, 0.06667, -0.26667, 0.33333 ]]
        ));
    }

}
