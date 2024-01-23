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

    // TODO: Return an error type from fire to avoid panicking on wrong paths?

    fn fire(&self, event: &Event, id_path: &IdPath) {
        self.body().fire(event, id_path);
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        self.bind(context);
        self.body().render(context)
    }
}

macro_rules! impl_tuple_view {
    ($($tvs:ident),*) => {
        impl<$($tvs),*> View for ($($tvs,)*) where $($tvs: View),* {
            fn fire(&self, event: &Event, id_path: &IdPath) {
                if let Some(head) = id_path.head() {
                    match head {
                        $(${ignore(tvs)} Id::Index(${index()}) => self.${index()}.fire(event, &id_path.tail()),)*
                        i => panic!("Cannot fire event for child id {} on a {}-tuple", i, ${count(tvs, 0)})
                    }
                }
            }

            fn render(&mut self, context: &Context) -> Identified<Node> {
                context.identify(Node::Group { children: vec![
                    $(${ignore(tvs)} self.${index()}.render(&context.child(${index()})),)*
                ] })
            }
        }
    };
}

impl View for ! {}

// Note: We explicitly implement the 0 (unit) and 1 tuples to avoid e.g. the
// overhead of using a Group requiring a Vec.

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

// TODO: Generate with variadic generics once available

impl_tuple_view!(T1, T2);
impl_tuple_view!(T1, T2, T3);
impl_tuple_view!(T1, T2, T3, T4);
impl_tuple_view!(T1, T2, T3, T4, T5);
impl_tuple_view!(T1, T2, T3, T4, T5, T6);
impl_tuple_view!(T1, T2, T3, T4, T5, T6, T7);
impl_tuple_view!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_tuple_view!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_tuple_view!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
