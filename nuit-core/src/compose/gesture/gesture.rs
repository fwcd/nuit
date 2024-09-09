use crate::{Bind, Context, EventResponse, GestureEvent, GestureNode, IdPath};

/// A composable gesture.
pub trait Gesture: Bind {
    type Body: Gesture = NeverGesture;

    fn body(&self) -> Self::Body {
        panic!("Gesture does not have a body!")
    }

    fn fire(&self, event: &GestureEvent, event_path: &IdPath, context: &Context) -> EventResponse {
        self.bind(context);
        self.body().fire(event, event_path, context)
    }

    fn render(&self, context: &Context) -> GestureNode {
        self.bind(context);
        self.body().render(context)
    }
}

/// A gesture type that can never be constructed.
pub enum NeverGesture {}

impl Bind for NeverGesture {}
impl Gesture for NeverGesture {}
