use serde::{Serialize, Deserialize};

/// Geometry for a view.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Frame {
    Constrained {
        min_width: Option<f64>,
        ideal_width: Option<f64>,
        max_width: Option<f64>,
        min_height: Option<f64>,
        ideal_height: Option<f64>,
        max_height: Option<f64>,
    },
    Exact {
        width: Option<f64>,
        height: Option<f64>,
    },
}

impl Frame {
    pub fn width(width: impl Into<f64>) -> Self {
        Self::Exact { width: Some(width.into()), height: None }
    }

    pub fn height(height: impl Into<f64>) -> Self {
        Self::Exact { width: None, height: Some(height.into()) }
    }

    pub fn exact(width: impl Into<f64>, height: impl Into<f64>) -> Self {
        Self::Exact { width: Some(width.into()), height: Some(height.into()) }
    }
}

impl From<i32> for Frame {
    fn from(value: i32) -> Self {
        Self::exact(value, value)
    }
}

impl From<f64> for Frame {
    fn from(value: f64) -> Self {
        Self::exact(value, value)
    }
}

impl<W, H> From<(W, H)> for Frame where W: Into<f64>, H: Into<f64> {
    fn from((width, height): (W, H)) -> Self {
        Self::exact(width, height)
    }
}
