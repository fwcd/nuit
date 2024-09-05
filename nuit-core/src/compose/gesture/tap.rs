use nuit_derive::Bind;

use crate::{Context, Gesture, GestureEvent, GestureNode, IdPath};

/// A gesture recognizing a tap.
#[derive(Bind)]
pub struct TapGesture<F> {
    count: usize,
    action: F,
}

impl<F> TapGesture<F> {
    /// Creates a tap gesture that executes the given action upon recognizing
    /// the given number of taps.
    pub fn new(count: usize, action: F) -> Self {
        Self { count, action }
    }

    /// Creates a tap gesture that executes the given action after recognizing a
    /// single tap.
    pub fn new_single(action: F) -> Self {
        Self::new(1, action)
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
        GestureNode::Tap { count: self.count }
    }
}
