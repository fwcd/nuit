use crate::{GestureEvent, GestureNode, IdPath};

/// A composable gesture.
pub trait Gesture {
    type Body: Gesture = NeverGesture;

    fn body(&self) -> Self::Body {
        panic!("Gesture does not have a body!")
    }

    fn fire(&self, event: &GestureEvent, id_path: &IdPath) {
        self.body().fire(event, id_path)
    }

    fn render(&self) -> GestureNode {
        Gesture::render(&self.body())
    }
}

/// A gesture type that can never be constructed.
pub enum NeverGesture {}

impl Gesture for NeverGesture {}
