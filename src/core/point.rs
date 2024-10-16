use crate::core::float::ApproxEq;
use crate::core::tuple::Tuple;
use crate::core::vector::Vector;

#[derive(Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Tuple for Point {
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
        1.0
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x.approx_eq(other.x) && self.y.approx_eq(other.y) && self.z.approx_eq(other.z)
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::Output::new(self.x() + rhs.x, self.y() + rhs.y, self.z() + rhs.z)
    }
}

impl std::ops::Sub for Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::new(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
}

impl std::ops::Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl std::ops::Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f64> for Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

mod tests {
    use super::*;

    #[test]
    fn point_has_w_value_0() {
        let point = Point::zero();
        assert_eq!(point.w(), 1.0);
    }

    #[test]
    fn negating_a_point() {
        // Given
        let p = Point::new(1.0, -2.0, 3.0);

        // Then
        assert_eq!(Point::new(-1.0, 2.0, -3.0), -p);
    }

    #[test]
    fn multiplying_a_point_by_a_scalar() {
        // Given
        let p = Point::new(1.0, -2.0, 3.0);

        // Then
        assert_eq!(Point::new(3.5, -7.0, 10.5), p.clone() * 3.5);
        assert_eq!(Point::new(3.5, -7.0, 10.5), 3.5 * p);
    }

    #[test]
    fn dividing_a_point_by_a_scalar() {
        // Given
        let p = Point::new(1.0, -2.0, 3.0);

        // Then
        assert_eq!(Point::new(0.5, -1.0, 1.5), p / 2.0);
    }
}