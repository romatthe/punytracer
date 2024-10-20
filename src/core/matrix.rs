use crate::core::float::ApproxEq;

struct Matrix2 {
    data: [f64; 4]
}

struct Matrix3 {
    data: [f64; 9]
}

struct Matrix4 {
    data: [f64; 16],
}

impl Matrix2 {
    pub fn new () -> Self {
        Self {
            data: [0.0; 4],
        }
    }
}

impl PartialEq for Matrix2 {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter().enumerate()
            .all(|(i, data)| data.approx_eq_low_precision(other.data[i]))
    }
}

impl std::ops::Index<(usize, usize)> for Matrix2 {
    type Output = f64;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x * 2 + y]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix2 {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x * 2 + y]
    }
}

impl Matrix3 {
    pub fn new () -> Self {
        Self {
            data: [0.0; 9],
        }
    }
}

impl PartialEq for Matrix3 {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter().enumerate()
            .all(|(i, data)| data.approx_eq_low_precision(other.data[i]))
    }
}

impl std::ops::Index<(usize, usize)> for Matrix3 {
    type Output = f64;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x * 3 + y]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix3 {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x * 3 + y]
    }
}

impl Matrix4 {
    pub fn new () -> Self {
        Self {
            data: [0.0; 16],
        }
    }
}

impl PartialEq for Matrix4 {
    fn eq(&self, other: &Self) -> bool {
        self.data
            .iter().enumerate()
            .all(|(i, data)| data.approx_eq_low_precision(other.data[i]))
    }
}

impl std::ops::Index<(usize, usize)> for Matrix4 {
    type Output = f64;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x * 4 + y]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix4 {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x * 4 + y]
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