use crate::core::float::ApproxEq;
use crate::core::point::Point;
use crate::core::tuple::Tuple;

#[derive(Clone, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl Tuple for Vector {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z, }
    }

    fn zero() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0}
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }

    fn w(&self) -> f64 {
        0.0
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl std::ops::Add for Vector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

mod tests {
    use super::*;

    #[test]
    fn vector_has_w_value_0() {
        let vector = Vector::zero();
        assert_eq!(vector.w(), 0.0);
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        // Given
        let v = Vector::new(1.0, -2.0, 3.0);

        // Then
        assert_eq!(Vector::new(-1.0, 2.0, -3.0), Vector::zero() - v);
    }

    #[test]
    fn negating_a_vector() {
        // Given
        let v = Vector::new(1.0, -2.0, 3.0);

        // Then
        assert_eq!(Vector::new(-1.0, 2.0, -3.0), -v);
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        // Given
        let v = Vector::new(1.0, -2.0, 3.0);

        // Then
        assert_eq!(Vector::new(3.5, -7.0, 10.5), v.clone() * 3.5);
        assert_eq!(Vector::new(3.5, -7.0, 10.5), 3.5 * v);
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        // Given
        let v = Vector::new(1.0, -2.0, 3.0);

        // Then
        assert_eq!(Vector::new(0.5, -1.0, 1.5), v / 2.0);
    }

    #[test]
    fn computing_magnitude_of_vector_1() {
        // Given
        let v = Vector::new(1.0, 0.0, 0.0);

        // Then
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_2() {
        // Given
        let v = Vector::new(0.0, 1.0, 0.0);

        // Then
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_3() {
        // Given
        let v = Vector::new(0.0, 0.0, 1.0);

        // Then
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_magnitude_of_vector_4() {
        // Given
        let v = Vector::new(1.0, 2.0, 3.0);

        // Then
        assert_eq!(v.magnitude(), 14f64.sqrt());
    }

    #[test]
    fn computing_magnitude_of_vector_5() {
        // Given
        let v = Vector::new(-1.0, -2.0, -3.0);

        // Then
        assert_eq!(v.magnitude(), 14f64.sqrt());
    }
}