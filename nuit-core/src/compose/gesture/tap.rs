use crate::{Gesture, GestureNode};

/// A gesture recognizing a tap.
pub struct TapGesture<F> {
    action: F,
}

impl<F> TapGesture<F> {
    pub fn new(action: F) -> Self {
        Self { action }
    }
}

impl<F> Gesture for TapGesture<F> where F: Fn() {
    fn render(&self) -> GestureNode {
        GestureNode::Tap {}
    }
}
