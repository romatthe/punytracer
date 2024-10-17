const EPSILON: f64 = 1.0e-7;
const EPSILON_LOW: f64 = 1.0e-3;

pub trait ApproxEq<Rhs = Self> {
    fn approx_eq(self, other: Rhs) -> bool;
    fn approx_eq_low_precision(self, other: Rhs) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, other: Self) -> bool {
        match (self - other).abs()  {
            n if n < EPSILON => true,
            _                => false,
        }
    }

    fn approx_eq_low_precision(self, other: Self) -> bool {
        match (self - other).abs()  {
            n if n < EPSILON_LOW => true,
            _                    => false,
        }
    }
}
