use nuit_derive::ApproxEq;
use serde::{Serialize, Deserialize};

/// Insets along each edge of a rectangle.
#[derive(Debug, Clone, Copy, PartialEq, ApproxEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Insets {
    pub top: f64,
    pub leading: f64,
    pub bottom: f64,
    pub trailing: f64,
}

impl Default for Insets {
    fn default() -> Self {
        Self {
            top: 10.0,
            leading: 10.0,
            bottom: 10.0,
            trailing: 10.0,
        }
    }
}

impl From<f64> for Insets {
    fn from(value: f64) -> Self {
        Self {
            top: value,
            leading: value,
            bottom: value,
            trailing: value,
        }
    }
}
