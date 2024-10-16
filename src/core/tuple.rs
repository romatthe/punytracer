pub trait Tuple {
    fn new(x: f64, y: f64, z: f64) -> Self;
    fn zero() -> Self;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
    fn w(&self) -> f64;
}

mod tests {
    use crate::core::point::Point;
    use crate::core::vector::Vector;
    use super::*;

    #[test]
    fn adding_two_tuples() {
        // Given
        let a = Point::new(3.0, -2.0, 5.0);
        let b = Vector::new(-2.0, 3.0, 1.0);

        // Then
        assert_eq!(Point::new(1.0, 1.0, 6.0,), a + b);
    }

    #[test]
    fn subtracting_two_points(){
        // Given
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        // Then
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), p1 - p2);
    }

    #[test]
    fn subtracting_vector_from_point(){
        // Given
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        // Then
        assert_eq!(Point::new(-2.0, -4.0, -6.0), p - v);
    }

    #[test]
    fn subtracting_two_vectors(){
        // Given
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        // Then
        assert_eq!(Vector::new(-2.0, -4.0, -6.0), v1 - v2);
    }
}
