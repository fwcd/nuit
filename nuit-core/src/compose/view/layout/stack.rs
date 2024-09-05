use nuit_derive::Bind;

use crate::{Context, Event, Id, IdPath, IdentifyExt, Node, View, DEFAULT_SPACING};

macro_rules! impl_stack {
    (#[doc = $doc:expr] $name:ident) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Bind)]
        pub struct $name<T> {
            spacing: f64,
            wrapped: T,
        }

        impl<T> $name<T> {
            #[doc = concat!("Creates a new ", stringify!($name), " from given wrapped view.")]
            pub const fn new(wrapped: T) -> Self {
                Self {
                    spacing: DEFAULT_SPACING,
                    wrapped,
                }
            }

            #[doc = concat!("Creates a new ", stringify!($name), " from given wrapped view with the given spacing.")]
            pub fn with_spacing(spacing: impl Into<f64>, wrapped: T) -> Self {
                Self {
                    spacing: spacing.into(),
                    wrapped,
                }
            }
        }

        impl<T> View for $name<T> where T: View {
            fn fire(&self, event: &Event, id_path: &IdPath) {
                if let Some(head) = id_path.head() {
                    match head {
                        Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                        i => panic!("Cannot fire event for child id {} on {} which only has one child", i, stringify!($name)),
                    }
                }
            }

            fn render(&self, context: &Context) -> Node {
                Node::$name {
                    spacing: self.spacing,
                    wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
                }
            }
        }
    };
}

impl_stack! {
    /// A view that lays out its children horizontally.
    HStack
}

impl_stack! {
    /// A view that lays out its children vertically.
    VStack
}

impl_stack! {
    /// A view that lays out its children on top of each other.
    ZStack
}
