use crate::{View, Primitive, Bind, Context};

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
    fn bind(&mut self, context: &Context) {
        self.wrapped.bind(context);
    }
}

impl<T> View for ZStack<T> where T: View {
    fn render(&mut self, context: &Context) -> Primitive {
        Primitive::ZStack { wrapped: Box::new(self.wrapped.render(&context.child(0))) }
    }
}
