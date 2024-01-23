mod button;
mod ext;
mod for_each;
mod h_stack;
mod r#if;
mod list;
mod modified;
mod text_field;
mod text;
mod v_stack;
mod z_stack;

pub use button::*;
pub use ext::*;
pub use for_each::*;
pub use h_stack::*;
pub use r#if::*;
pub use list::*;
pub use modified::*;
pub use text_field::*;
pub use text::*;
pub use v_stack::*;
pub use z_stack::*;

use crate::{Node, Bind, Context, Identified, Event, IdPath, Id};

/// The primary view trait. Represents a lightweight UI component.
pub trait View: Bind {
    type Body: View = !;

    fn body(&self) -> Self::Body {
        panic!("View does not have a body!")
    }
    
    fn fire(&self, event: &Event, id_path: &IdPath) {
        self.body().fire(event, id_path);
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        self.bind(context);
        self.body().render(context)
    }
}

// TODO: Generate tuple variants with a macro (or variadic generics once possible)

impl View for ! {}

impl View for () {
    fn fire(&self, _event: &Event, _id_path: &IdPath) {}

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Empty {})
    }
}

impl<T> View for (T,) where T: View {
    type Body = T::Body;

    fn body(&self) -> Self::Body {
        self.0.body()
    }

    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.0.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on 1-tuple", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        self.0.render(&context.child(0))
    }
}

impl<T, U> View for (T, U) where T: View, U: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.0.fire(event, &id_path.tail()),
                Id::Index(1) => self.1.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on 2-tuple", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Group { children: vec![
            self.0.render(&context.child(0)),
            self.1.render(&context.child(1)),
        ] })
    }
}

impl<T, U, V> View for (T, U, V) where T: View, U: View, V: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.0.fire(event, &id_path.tail()),
                Id::Index(1) => self.1.fire(event, &id_path.tail()),
                Id::Index(2) => self.2.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on 3-tuple", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Group { children: vec![
            self.0.render(&context.child(0)),
            self.1.render(&context.child(1)),
            self.2.render(&context.child(2)),
        ] })
    }
}

impl<T, U, V, W> View for (T, U, V, W) where T: View, U: View, V: View, W: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.0.fire(event, &id_path.tail()),
                Id::Index(1) => self.1.fire(event, &id_path.tail()),
                Id::Index(2) => self.2.fire(event, &id_path.tail()),
                Id::Index(3) => self.3.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on 4-tuple", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Group { children: vec![
            self.0.render(&context.child(0)),
            self.1.render(&context.child(1)),
            self.2.render(&context.child(2)),
            self.3.render(&context.child(3)),
        ] })
    }
}

impl<T, U, V, W, X> View for (T, U, V, W, X) where T: View, U: View, V: View, W: View, X: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.0.fire(event, &id_path.tail()),
                Id::Index(1) => self.1.fire(event, &id_path.tail()),
                Id::Index(2) => self.2.fire(event, &id_path.tail()),
                Id::Index(3) => self.3.fire(event, &id_path.tail()),
                Id::Index(4) => self.4.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on 5-tuple", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Group { children: vec![
            self.0.render(&context.child(0)),
            self.1.render(&context.child(1)),
            self.2.render(&context.child(2)),
            self.3.render(&context.child(3)),
            self.4.render(&context.child(4)),
        ] })
    }
}
