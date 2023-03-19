mod h_stack;
mod text;
mod v_stack;
mod z_stack;

pub use h_stack::*;
pub use text::*;
pub use v_stack::*;
pub use z_stack::*;

use crate::{Never, Primitive};

/// The primary view trait. Represents a lightweight UI component.
pub trait View {
    type Body: View;

    fn body(&self) -> Self::Body {
        panic!("View does not have a body!")
    }

    fn primitive(&self) -> Primitive {
        self.body().primitive()
    }
}

impl View for Never {
    type Body = Never;
}

// TODO: Generate tuple variants with a macro (or variadic generics once possible)

impl<T, U> View for (T, U) where T: View, U: View {
    type Body = Never;

    fn primitive(&self) -> Primitive {
        Primitive::Tuple2 {
            child1: Box::new(self.0.primitive()),
            child2: Box::new(self.1.primitive()),
        }
    }
}

impl<T, U, V> View for (T, U, V) where T: View, U: View, V: View {
    type Body = Never;

    fn primitive(&self) -> Primitive {
        Primitive::Tuple3 {
            child1: Box::new(self.0.primitive()),
            child2: Box::new(self.1.primitive()),
            child3: Box::new(self.2.primitive()),
        }
    }
}

impl<T, U, V, W> View for (T, U, V, W) where T: View, U: View, V: View, W: View {
    type Body = Never;

    fn primitive(&self) -> Primitive {
        Primitive::Tuple4 {
            child1: Box::new(self.0.primitive()),
            child2: Box::new(self.1.primitive()),
            child3: Box::new(self.2.primitive()),
            child4: Box::new(self.3.primitive()),
        }
    }
}

impl<T, U, V, W, X> View for (T, U, V, W, X) where T: View, U: View, V: View, W: View, X: View {
    type Body = Never;

    fn primitive(&self) -> Primitive {
        Primitive::Tuple5 {
            child1: Box::new(self.0.primitive()),
            child2: Box::new(self.1.primitive()),
            child3: Box::new(self.2.primitive()),
            child4: Box::new(self.3.primitive()),
            child5: Box::new(self.4.primitive()),
        }
    }
}
