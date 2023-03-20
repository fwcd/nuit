use crate::{View, Primitive, Bind, Context};

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
    fn bind(&mut self, context: &Context) {
        self.wrapped.bind(context);
    }
}

impl<T> View for VStack<T> where T: View {
    fn render(&mut self, context: &Context) -> Primitive {
        Primitive::VStack { wrapped: Box::new(self.wrapped.render(&context.child(0))) }
    }
}
