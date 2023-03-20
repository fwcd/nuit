use crate::{Storage, Primitive, View};

/// The central state of a NUI application.
pub struct NUIRoot<T> {
    view: T,
    storage: Storage,
}

impl<T> NUIRoot<T> {
    pub fn new(view: T) -> Self {
        Self {
            view,
            storage: Storage::new(),
        }
    }
}

impl<T> NUIRoot<T> where T: View {
    pub fn render(&self) -> Primitive {
        self.view.render(&self.storage)
    }
}
