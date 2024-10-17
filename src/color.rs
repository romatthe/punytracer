use crate::core::float::ApproxEq;

#[derive(Clone, Debug, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        let r = match self.r {
            r if r < 0.0 => 0u8,
            r if r > 1.0 => 255u8,
            r => (r * 255.0) as u8,
        };
        let g = match self.g {
            g if g < 0.0 => 0u8,
            g if g > 1.0 => 255u8,
            g => (g * 255.0) as u8,
        };
        let b = match self.b {
            b if b < 0.0 => 0u8,
            b if b > 1.0 => 255u8,
            b => (b * 255.0) as u8,
        };

        [r, g, b]
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r.approx_eq_low_precision(other.r)
            && self.g.approx_eq_low_precision(other.g)
            && self.b.approx_eq_low_precision(other.b)
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.g - rhs.g, self.b - rhs.b)
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

mod tests {
    use super::*;

    #[test]
    fn create_color() {
        // Given
        let color = Color::new(-0.5, 0.4, 1.7);

        // Then
        assert_eq!(color.r, -0.5);
        assert_eq!(color.g, 0.4);
        assert_eq!(color.b, 1.7);
    }

    #[test]
    fn add_two_colors() {
        // Given
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        // Then
        assert_eq!(Color::new(1.6, 0.7, 1.0), c1 + c2);
    }

    #[test]
    fn subtract_two_colors() {
        // Given
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        // Then
        assert_eq!(Color::new(0.2, 0.5, 0.5), c1 - c2);
    }

    #[test]
    fn multiply_color_by_scalar() {
        // Given
        let color = Color::new(0.2, 0.3, 0.4);

        // Then
        assert_eq!(Color::new(0.4, 0.6, 0.8), color * 2.0);
    }

    #[test]
    fn multiply_two_colors() {
        // Given
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        // Then
        assert_eq!(Color::new(0.9, 0.2, 0.04), c1 * c2);
    }
}