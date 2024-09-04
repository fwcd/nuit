use std::{f64::consts::PI, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use serde::{Deserialize, Serialize};

/// A geometric angle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
    pub const fn with_radians(radians: f64) -> Self {
        Self { radians }
    }

    /// Creates an angle from the given value in degrees.
    pub const fn with_degrees(degrees: f64) -> Self {
        Self::with_radians(degrees * PI / 180.0)
    }

    /// The value in radians.
    pub fn radians(self) -> f64 {
        self.radians
    }

    /// The value in degrees.
    pub fn degrees(self) -> f64 {
        self.radians * 180.0 / PI
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
