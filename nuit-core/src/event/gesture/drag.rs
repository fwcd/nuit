use serde::{Deserialize, Serialize};

use crate::Vec2;

/// An event emitted during a drag gesture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragEvent {
    start_location: Vec2<f64>,
    location: Vec2<f64>,
}

impl DragEvent {
    /// The starting point of the drag.
    pub fn start_location(&self) -> Vec2<f64> {
        self.start_location
    }

    /// The current point of the drag.
    pub fn location(&self) -> Vec2<f64> {
        self.location
    }

    /// The distance moved during the gesture.
    pub fn translation(&self) -> Vec2<f64> {
        self.location - self.start_location
    }
}
