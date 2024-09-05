use nuit_derive::Bind;

use crate::{Context, Gesture, GestureEvent, GestureNode, IdPath};

/// A gesture recognizing a tap.
#[derive(Bind)]
pub struct TapGesture<F> {
    action: F,
}

impl<F> TapGesture<F> {
    pub fn new(action: F) -> Self {
        Self { action }
    }
}

impl<F> Gesture for TapGesture<F> where F: Fn() {
    fn fire(&self, event: &GestureEvent, id_path: &IdPath) {
        if let GestureEvent::Tap {} = event {
            (self.action)();
        } else {
            eprintln!("Warning: Ignoring non-tap gesture event {:?} targeted to TapGesture at {:?}", event, id_path)
        }
    }

    fn render(&self, _context: &Context) -> GestureNode {
        GestureNode::Tap {}
    }
}
