use crate::{View, Primitive, Bind, Storage};

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

impl<T> Bind for ZStack<T> where T: Bind {
    fn bind(&self, storage: &Storage) {
        self.wrapped.bind(storage);
    }
}

impl<T> View for ZStack<T> where T: View {
    fn primitive(&self) -> Primitive {
        Primitive::ZStack { wrapped: Box::new(self.wrapped.primitive()) }
    }
}
