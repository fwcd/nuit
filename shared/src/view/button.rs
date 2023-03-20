use crate::{View, Primitive, Bind, Context};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button<T, F> {
    label: T,
    action: F,
}

impl<T, F> Button<T, F> {
    pub fn new(label: T, action: F) -> Self {
        Self {
            label,
            action,
        }
    }
}

impl<T, F> Bind for Button<T, F> where T: Bind, F: Fn() {}

impl<T, F> View for Button<T, F> where T: View, F: Fn() {
    fn render(&mut self, context: &Context) -> Primitive {
        Primitive::Button { label: Box::new(self.label.render(&context.child(0))) }
    }
}
