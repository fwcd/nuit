use nuit_derive::Bind;

use crate::{Context, DragEvent, GestureEvent, GestureNode, IdPath};

use super::Gesture;

/// A gesture recognizing a drag.
#[derive(Bind)]
pub struct DragGesture<F> {
    minimum_distance: f64,
    action: F,
}

impl<F> DragGesture<F> {
    /// Creates a drag gesture that executes the given action on a drag further
    /// than the given minimum distance.
    pub const fn new(minimum_distance: f64, action: F) -> Self {
        Self { minimum_distance, action }
    }

    /// Creates a drag gesture with the default minimum distance.
    pub const fn new_default(action: F) -> Self {
        Self { minimum_distance: 10.0, action }
    }
}

impl<F> Gesture for DragGesture<F> where F: Fn(&DragEvent) {
    fn fire(&self, event: &GestureEvent, event_path: &IdPath, _context: &Context) {
        assert!(event_path.is_root());
        if let GestureEvent::Drag { drag } = event {
            (self.action)(drag);
        } else {
            eprintln!("Warning: Ignoring non-drag gesture event {:?} targeted to DragGesture at {:?}", event, event_path)
        }
    }

    fn render(&self, _context: &Context) -> GestureNode {
        GestureNode::Drag { minimum_distance: self.minimum_distance }
    }
}
