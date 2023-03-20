use crate::{View, Primitive, Bind, Storage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button<T> {
    label: T,
}

impl<T> Button<T> {
    pub fn new(label: T) -> Self {
        Self {
            label
        }
    }
}

impl<T> Bind for Button<T> where T: Bind {}

impl<T> View for Button<T> where T: View {
    fn render(&self, storage: &Storage) -> Primitive {
        Primitive::Button { label: Box::new(self.label.render(storage)) }
    }
}
