use std::f64::consts::PI;

use serde::{Deserialize, Serialize};

/// A geometric angle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Angle {
    radians: f64,
}

impl Angle {
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
