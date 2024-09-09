use nuit_derive::Bind;

use crate::{Alignment, Context, Event, EventResponse, HorizontalAlignment, Id, IdPath, IdentifyExt, Node, VerticalAlignment, View, DEFAULT_SPACING};

macro_rules! impl_stack {
    (#[doc = $doc:expr] $name:ident, $alignment:ident) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Bind)]
        pub struct $name<T> {
            alignment: $alignment,
            spacing: f64,
            wrapped: T,
        }

        impl<T> $name<T> {
            #[doc = concat!("Creates a new ", stringify!($name), " with the given alignment, spacing and wrapped view.")]
            #[must_use]
            pub fn new(alignment: impl Into<$alignment>, spacing: impl Into<f64>, wrapped: T) -> Self {
                Self {
                    alignment: alignment.into(),
                    spacing: spacing.into(),
                    wrapped,
                }
            }

            #[doc = concat!("Creates a new ", stringify!($name), " with the given spacing and wrapped view.")]
            #[must_use]
            pub fn with_spacing(spacing: impl Into<f64>, wrapped: T) -> Self {
                Self {
                    alignment: Default::default(),
                    spacing: spacing.into(),
                    wrapped,
                }
            }

            #[doc = concat!("Creates a new ", stringify!($name), " with the given alignment and wrapped view.")]
            #[must_use]
            pub fn with_alignment(alignment: impl Into<$alignment>, wrapped: T) -> Self {
                Self {
                    alignment: alignment.into(),
                    spacing: DEFAULT_SPACING,
                    wrapped,
                }
            }
        }

        impl<T> From<T> for $name<T> {
            fn from(wrapped: T) -> Self {
                Self {
                    alignment: Default::default(),
                    spacing: DEFAULT_SPACING,
                    wrapped,
                }
            }
        }

        impl<T> View for $name<T> where T: View {
            fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) -> EventResponse {
                if let Some(head) = event_path.head() {
                    match head {
                        Id::Index(0) => self.wrapped.fire(event, &event_path.tail(), &context.child(0)),
                        i => panic!("Cannot fire event for child id {} on {} which only has one child", i, stringify!($name)),
                    }
                } else {
                    EventResponse::default()
                }
            }

            fn render(&self, context: &Context) -> Node {
                Node::$name {
                    alignment: self.alignment,
                    spacing: self.spacing,
                    wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
                }
            }
        }
    };
}

impl_stack! {
    /// A view that lays out its children horizontally.
    HStack, VerticalAlignment
}

impl_stack! {
    /// A view that lays out its children vertically.
    VStack, HorizontalAlignment
}

impl_stack! {
    /// A view that lays out its children on top of each other.
    ZStack, Alignment
}
