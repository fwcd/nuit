use std::{f64::consts::PI, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use nuit_derive::ApproxEq;
use serde::{Deserialize, Serialize};

/// A geometric angle.
#[derive(Debug, Default, Clone, Copy, PartialEq, ApproxEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Angle {
    radians: f64,
}

impl Angle {
    pub const ZERO: Self = Self::with_radians(0.0);
    pub const QUARTER: Self = Self::with_radians(PI / 2.0);
    pub const HALF: Self = Self::with_radians(PI);
    pub const FULL: Self = Self::with_radians(2.0 * PI);

    /// Creates an angle from the given value in radians.
    #[must_use]
    pub const fn with_radians(radians: f64) -> Self {
        Self { radians }
    }

    /// Creates an angle from the given value in degrees.
    #[must_use]
    pub const fn with_degrees(degrees: f64) -> Self {
        Self::with_radians(degrees * PI / 180.0)
    }

    /// Creates an angle from the given fractional value between 0 and 1.
    #[must_use]
    pub const fn with_fractional(fractional: f64) -> Self {
        Self::with_radians(fractional * 2.0 * PI)
    }

    /// The value in radians.
    #[must_use]
    pub const fn radians(self) -> f64 {
        self.radians
    }

    /// The value in degrees.
    #[must_use]
    pub const fn degrees(self) -> f64 {
        self.radians * 180.0 / PI
    }

    /// The fractional value between 0 and 1.
    #[must_use]
    pub const fn fractional(self) -> f64 {
        self.radians / (2.0 * PI)
    }
}

impl Add for Angle {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { radians: self.radians + rhs.radians }
    }
}

impl Sub for Angle {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self { radians: self.radians - rhs.radians }
    }
}

impl Mul<f64> for Angle {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self { radians: self.radians * rhs }
    }
}

impl Div<f64> for Angle {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self { radians: self.radians / rhs }
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, rhs: Self) {
        self.radians += rhs.radians;
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, rhs: Self) {
        self.radians -= rhs.radians;
    }
}

impl MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        self.radians *= rhs;
    }
}

impl DivAssign<f64> for Angle {
    fn div_assign(&mut self, rhs: f64) {
        self.radians /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{assert_approx_eq, Angle};

    #[test]
    fn conversions() {
        assert_approx_eq!(Angle::HALF.fractional(), 0.5);
        assert_approx_eq!(Angle::HALF.degrees(), 180.0);
        assert_approx_eq!(Angle::HALF.radians(), PI);

        assert_approx_eq!(Angle::with_fractional(1.0), Angle::FULL);
        assert_approx_eq!(Angle::with_degrees(360.0), Angle::FULL);
        assert_approx_eq!(Angle::with_radians(2.0 * PI), Angle::FULL);
    }
}
