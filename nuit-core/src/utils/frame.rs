use serde::{Serialize, Deserialize};

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

impl From<(f64, f64)> for Frame {
    fn from((width, height): (f64, f64)) -> Self {
        Self::exact(width, height)
    }
}
