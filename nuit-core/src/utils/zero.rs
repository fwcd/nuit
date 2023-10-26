/// A type that has a zero value.
pub trait Zero {
    /// The zero value.
    const ZERO: Self;
}

macro_rules! impl_int_zero {
    ($($tys:ty),*) => {
        $(impl Zero for $tys {
            const ZERO: Self = 0;
        })*
    };
}

macro_rules! impl_float_zero {
    ($($tys:ty),*) => {
        $(impl Zero for $tys {
            const ZERO: Self = 0.0;
        })*
    };
}

impl_int_zero!(
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128
);

impl_float_zero!(
    f32, f64
);
