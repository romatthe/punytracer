use crate::core::float::ApproxEq;
use crate::core::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
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
        self.x.approx_eq_low_precision(other.x)
            && self.y.approx_eq_low_precision(other.y)
            && self.z.approx_eq_low_precision(other.z)
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

    #[test]
    fn normalizing_a_vector_1() {
        // Given
        let v = Vector::new(4.0, 0.0, 0.0);

        // Then
        assert_eq!(Vector::new(1.0, 0.0, 0.0), v.normalize());
    }

    #[test]
    fn normalizing_a_vector_2() {
        // Given
        let v = Vector::new(1.0, 2.0, 3.0);

        // Then
        assert_eq!(Vector::new(0.26726, 0.53452, 0.80178), v.normalize());
        //                         1/√14,      2/√14,      3/√14
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        // Given
        let v = Vector::new(1.0, 2.0, 3.0);

        // Then
        assert_eq!(v.normalize().magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_vectors() {
        // Given
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        // Then
        assert_eq!(v1.dot(v2), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        // Given
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);

        // Then
        assert_eq!(v1.cross(v2), Vector::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1), Vector::new(1.0, -2.0, 1.0));
    }
}