use crate::core::float::ApproxEq;

pub type Matrix2 = Matrix<2>;
pub type Matrix3 = Matrix<3>;
pub type Matrix4 = Matrix<4>;

struct Matrix<const N: usize> {
    data: [[f64; N]; N]
}

impl<const N: usize> Matrix<N> {
    pub fn new() -> Self {
        Self { data: [[0.0; N]; N], }
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

mod tests {
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
}