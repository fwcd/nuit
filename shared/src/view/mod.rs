mod h_stack;
mod text;
mod v_stack;
mod z_stack;

pub use h_stack::*;
pub use text::*;
pub use v_stack::*;
pub use z_stack::*;

use crate::{Primitive, Bind, Storage};

/// The primary view trait. Represents a lightweight UI component.
pub trait View: Bind {
    type Body: View = !;

    fn body(&self) -> Self::Body {
        panic!("View does not have a body!")
    }

    fn render(&self, storage: &Storage) -> Primitive {
        self.body().render(storage)
    }
}

// TODO: Generate tuple variants with a macro (or variadic generics once possible)

impl View for ! {}

impl View for () {
    fn render(&self, _storage: &Storage) -> Primitive {
        Primitive::Empty
    }
}

impl<T> View for (T,) where T: View {
    type Body = T::Body;

    fn body(&self) -> Self::Body {
        self.0.body()
    }

    fn render(&self, storage: &Storage) -> Primitive {
        self.0.render(storage)
    }
}

impl<T, U> View for (T, U) where T: View, U: View {
    fn render(&self, storage: &Storage) -> Primitive {
        Primitive::Tuple2 {
            child1: Box::new(self.0.render(storage)),
            child2: Box::new(self.1.render(storage)),
        }
    }
}

impl<T, U, V> View for (T, U, V) where T: View, U: View, V: View {
    fn render(&self, storage: &Storage) -> Primitive {
        Primitive::Tuple3 {
            child1: Box::new(self.0.render(storage)),
            child2: Box::new(self.1.render(storage)),
            child3: Box::new(self.2.render(storage)),
        }
    }
}

impl<T, U, V, W> View for (T, U, V, W) where T: View, U: View, V: View, W: View {
    fn render(&self, storage: &Storage) -> Primitive {
        Primitive::Tuple4 {
            child1: Box::new(self.0.render(storage)),
            child2: Box::new(self.1.render(storage)),
            child3: Box::new(self.2.render(storage)),
            child4: Box::new(self.3.render(storage)),
        }
    }
}

impl<T, U, V, W, X> View for (T, U, V, W, X) where T: View, U: View, V: View, W: View, X: View {
    fn render(&self, storage: &Storage) -> Primitive {
        Primitive::Tuple5 {
            child1: Box::new(self.0.render(storage)),
            child2: Box::new(self.1.render(storage)),
            child3: Box::new(self.2.render(storage)),
            child4: Box::new(self.3.render(storage)),
            child5: Box::new(self.4.render(storage)),
        }
    }
}
