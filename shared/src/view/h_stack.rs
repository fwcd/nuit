use crate::{View, Primitive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HStack<T> {
    wrapped: T,
}

impl<T> HStack<T> {
    pub fn new(wrapped: T) -> Self {
        Self {
            wrapped
        }
    }
}

impl<T> View for HStack<T> where T: View {
    fn primitive(&self) -> Primitive {
        Primitive::HStack { wrapped: Box::new(self.wrapped.primitive()) }
    }
}
