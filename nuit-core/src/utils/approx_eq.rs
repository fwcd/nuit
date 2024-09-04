/// A trait for approximate equality.
pub trait ApproxEq<Rhs = Self> {
    /// Whether the value is within the tolerance of the other one.
    fn approx_eq(&self, other: &Rhs, tolerance: f64) -> bool;
}

macro_rules! impl_primitive_approx_eq {
    ($($tys:ty),*) => {
        $(impl ApproxEq for $tys {
            fn approx_eq(&self, other: &Self, tolerance: f64) -> bool {
                ((self - other) as f64).abs() <= tolerance
            }
        })*
    };
}

#[macro_export]
macro_rules! assert_approx_eq {
    ($e1:expr, $e2:expr, $tolerance:expr) => {
        assert!($crate::ApproxEq::approx_eq(&$e1, &$e2, $tolerance), "Assertion failed: Not approximately equal\n  Left:  {:?}\n  Right: {:?}",  $e1, $e2);
    };
    ($e1:expr, $e2:expr) => {
        assert_approx_eq!($e1, $e2, 0.00000001);
    };
}

impl_primitive_approx_eq!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);
