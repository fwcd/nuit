use crate::{View, Primitive, Bind, Storage};

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

impl<T> Bind for VStack<T> where T: Bind {
    fn bind(&self, storage: &Storage) {
        self.wrapped.bind(storage);
    }
}

impl<T> View for VStack<T> where T: View {
    fn render(&self, storage: &Storage) -> Primitive {
        Primitive::VStack { wrapped: Box::new(self.wrapped.render(storage)) }
    }
}
