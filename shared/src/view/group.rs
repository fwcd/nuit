use crate::{View, Never, Primitive};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group<T> {
    children: Vec<T>,
}

impl<T> Group<T> {
    pub fn new(children: impl IntoIterator<Item = T>) -> Self {
        Self {
            children: children.into_iter().collect()
        }
    }
}

impl<T> View for Group<T> where T: View {
    type Body = Never;

    fn primitive(&self) -> Primitive {
        Primitive::Group { children: self.children.iter().map(|c| c.primitive()).collect() }
    }
}
