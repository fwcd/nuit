use crate::{View, Primitive, Bind, Storage};

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

impl<T> Bind for HStack<T> where T: Bind {
    fn bind(&self, storage: &Storage) {
        self.wrapped.bind(storage);
    }
}

impl<T> View for HStack<T> where T: View {
    fn render(&self, storage: &Storage) -> Primitive {
        Primitive::HStack { wrapped: Box::new(self.wrapped.render(storage)) }
    }
}
