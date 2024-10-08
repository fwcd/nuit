use nuit_derive::Bind;

use crate::{View, Node, Context, Event, EventResponse, IdPath, Id, IdentifyExt};

/// A conditional view that can take on one of at most two branches, depending
/// on a boolean condition.
#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct If<T, F> {
    condition: bool,
    then_view: Option<T>,
    else_view: Option<F>,
}

impl<T> If<T, ()> {
    pub fn new(condition: bool, then_view: impl FnOnce() -> T) -> Self {
        Self {
            condition,
            then_view: if condition { Some(then_view()) } else { None },
            else_view: None,
        }
    }
}

#[allow(clippy::if_not_else)]
impl<T, F> If<T, F> {
    pub fn new_or_else(condition: bool, then_view: impl FnOnce() -> T, else_view: impl FnOnce() -> F) -> Self {
        Self {
            condition,
            then_view: if condition { Some(then_view()) } else { None },
            else_view: if !condition { Some(else_view()) } else { None },
        }
    }

    pub fn or_else<G>(self, else_view: impl FnOnce() -> G) -> If<T, G> {
        If {
            condition: self.condition,
            then_view: self.then_view,
            else_view: if !self.condition { Some(else_view()) } else { None },
        }
    }
}

impl<T, F> View for If<T, F> where T: View, F: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) -> EventResponse {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => {
                    if let Some(ref then_view) = self.then_view {
                        return then_view.fire(event, event_path.tail(), &context.child(0));
                    }
                },
                Id::Index(1) => {
                    if let Some(ref else_view) = self.else_view {
                        return else_view.fire(event, event_path.tail(), &context.child(1));
                    }
                },
                i => panic!("Cannot fire event for child id {i} on HStack which only has two childs"),
            }
        }
        EventResponse::default()
    }

    fn render(&self, context: &Context) -> Node {
        if let Some(ref then_view) = self.then_view {
            Node::Child { wrapped: Box::new(then_view.render(&context.child(0)).identify(0)) }
        } else if let Some(ref else_view) = self.else_view {
            Node::Child { wrapped: Box::new(else_view.render(&context.child(1)).identify(1)) }
        } else {
            Node::Empty {}
        }
    }
}
