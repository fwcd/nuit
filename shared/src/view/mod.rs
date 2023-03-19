mod h_stack;
mod primitive;
mod text;
mod v_stack;
mod z_stack;

pub use h_stack::*;
pub use primitive::*;
pub use text::*;
pub use v_stack::*;
pub use z_stack::*;

use crate::Never;

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
