mod button;
mod h_stack;
mod text_field;
mod text;
mod v_stack;
mod z_stack;

pub use button::*;
pub use h_stack::*;
pub use text_field::*;
pub use text::*;
pub use v_stack::*;
pub use z_stack::*;

use crate::{Node, Bind, Context, Id};

/// The primary view trait. Represents a lightweight UI component.
pub trait View: Bind {
    type Body: View = !;

    fn body(&self) -> Self::Body {
        panic!("View does not have a body!")
    }

    fn render(&mut self, context: &Context) -> Id<Node> {
        self.bind(context);
        self.body().render(context)
    }
}

// TODO: Generate tuple variants with a macro (or variadic generics once possible)

impl View for ! {}

impl View for () {
    fn render(&mut self, context: &Context) -> Id<Node> {
        context.identify(Node::Empty)
    }
}

impl<T> View for (T,) where T: View {
    type Body = T::Body;

    fn body(&self) -> Self::Body {
        self.0.body()
    }

    fn render(&mut self, context: &Context) -> Id<Node> {
        self.0.render(&context.child(0))
    }
}

impl<T, U> View for (T, U) where T: View, U: View {
    fn render(&mut self, context: &Context) -> Id<Node> {
        context.identify(Node::Tuple2 {
            child1: Box::new(self.0.render(&context.child(0))),
            child2: Box::new(self.1.render(&context.child(1))),
        })
    }
}

impl<T, U, V> View for (T, U, V) where T: View, U: View, V: View {
    fn render(&mut self, context: &Context) -> Id<Node> {
        context.identify(Node::Tuple3 {
            child1: Box::new(self.0.render(&context.child(0))),
            child2: Box::new(self.1.render(&context.child(1))),
            child3: Box::new(self.2.render(&context.child(2))),
        })
    }
}

impl<T, U, V, W> View for (T, U, V, W) where T: View, U: View, V: View, W: View {
    fn render(&mut self, context: &Context) -> Id<Node> {
        context.identify(Node::Tuple4 {
            child1: Box::new(self.0.render(&context.child(0))),
            child2: Box::new(self.1.render(&context.child(1))),
            child3: Box::new(self.2.render(&context.child(2))),
            child4: Box::new(self.3.render(&context.child(3))),
        })
    }
}

impl<T, U, V, W, X> View for (T, U, V, W, X) where T: View, U: View, V: View, W: View, X: View {
    fn render(&mut self, context: &Context) -> Id<Node> {
        context.identify(Node::Tuple5 {
            child1: Box::new(self.0.render(&context.child(0))),
            child2: Box::new(self.1.render(&context.child(1))),
            child3: Box::new(self.2.render(&context.child(2))),
            child4: Box::new(self.3.render(&context.child(3))),
            child5: Box::new(self.4.render(&context.child(4))),
        })
    }
}
