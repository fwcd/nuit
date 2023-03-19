use crate::{View, Never, Primitive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZStack<T> {
    wrapped: T,
}

impl<T> ZStack<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> View for ZStack<T> where T: View {
    type Body = Never;

    fn primitive(&self) -> Primitive {
        Primitive::ZStack { wrapped: Box::new(self.wrapped.primitive()) }
    }
}
