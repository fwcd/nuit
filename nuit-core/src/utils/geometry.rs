use serde::{Deserialize, Serialize};

use super::Vec2;

/// The rendered geometry of a view.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    size: Vec2<f64>,
}

impl Geometry {
    #[must_use]
    pub const fn size(self) -> Vec2<f64> {
        self.size
    }
}
