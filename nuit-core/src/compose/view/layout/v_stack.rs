use crate::{Bind, Context, Event, Id, IdPath, IdentifyExt, Node, View, DEFAULT_SPACING};

/// A view that lays out its children vertically.
#[derive(Debug, Clone, PartialEq)]
pub struct VStack<T> {
    spacing: f64,
    wrapped: T,
}

impl<T> VStack<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            spacing: DEFAULT_SPACING,
            wrapped,
        }
    }

    pub fn with_spacing(spacing: impl Into<f64>, wrapped: T) -> Self {
        Self {
            spacing: spacing.into(),
            wrapped,
        }
    }
}

impl<T> Bind for VStack<T> where T: Bind {}

impl<T> View for VStack<T> where T: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on VStack which only has one child", i)
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::VStack {
            spacing: self.spacing,
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
        }
    }
}
