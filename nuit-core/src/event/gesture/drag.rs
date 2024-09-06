use serde::{Deserialize, Serialize};

use crate::Vec2;

/// An event emitted during a drag gesture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragEvent {
    kind: DragEventKind,
    start_location: Vec2<f64>,
    location: Vec2<f64>,
}

impl DragEvent {
    /// The kind of the drag event.
    pub fn kind(&self) -> DragEventKind {
        self.kind
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DragEventKind {
    Updated,
    Ended,
}
