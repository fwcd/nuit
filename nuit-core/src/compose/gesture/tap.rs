use nuit_derive::Bind;

use crate::{Context, EventResponse, Gesture, GestureEvent, GestureNode, IdPath};

/// A gesture recognizing a tap.
#[derive(Bind)]
pub struct TapGesture<F> {
    count: usize,
    action: F,
}

impl<F> TapGesture<F> where F: Fn() {
    /// Creates a tap gesture that executes the given action upon recognizing
    /// the given number of taps.
    pub const fn new(count: usize, action: F) -> Self {
        Self { count, action }
    }

    /// Creates a tap gesture that executes the given action after recognizing a
    /// single tap.
    pub const fn new_single(action: F) -> Self {
        Self::new(1, action)
    }
}

impl<F> Gesture for TapGesture<F> where F: Fn() {
    fn fire(&self, event: &GestureEvent, event_path: &IdPath, _context: &Context) -> EventResponse {
        assert!(event_path.is_root());
        if let GestureEvent::Tap {} = event {
            (self.action)();
        } else {
            eprintln!("Warning: Ignoring non-tap gesture event {event:?} targeted to TapGesture at {event_path:?}");
        }
        EventResponse::default()
    }

    fn render(&self, _context: &Context) -> GestureNode {
        GestureNode::Tap { count: self.count }
    }
}
