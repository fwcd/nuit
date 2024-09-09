use serde::{Serialize, Deserialize};

/// Geometry constraints for layouting a view.
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
    #[must_use]
    pub fn with_width(width: impl Into<f64>) -> Self {
        Self::Exact { width: Some(width.into()), height: None }
    }

    #[must_use]
    pub fn with_height(height: impl Into<f64>) -> Self {
        Self::Exact { width: None, height: Some(height.into()) }
    }

    #[must_use]
    pub fn exact(width: impl Into<f64>, height: impl Into<f64>) -> Self {
        Self::Exact { width: Some(width.into()), height: Some(height.into()) }
    }
}

macro_rules! impl_exact_from {
    ($($tys:ty),*) => {
        $(impl From<$tys> for Frame {
            #[allow(clippy::cast_lossless, clippy::cast_precision_loss)]
            fn from(value: $tys) -> Self {
                Self::exact(value as f64, value as f64)
            }
        })*
    };
}

impl_exact_from!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64
);

impl<W, H> From<(W, H)> for Frame where W: Into<f64>, H: Into<f64> {
    fn from((width, height): (W, H)) -> Self {
        Self::exact(width, height)
    }
}
