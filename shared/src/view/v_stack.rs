use crate::{View, Never, Primitive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VStack<T> {
    wrapped: T,
}

impl<T> VStack<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> View for VStack<T> where T: View {
    type Body = Never;

    fn primitive(&self) -> Primitive {
        Primitive::VStack(Box::new(self.wrapped.primitive()))
    }
}
