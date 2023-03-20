use std::rc::Rc;

use crate::{Storage, Primitive, View, Context, Id};

/// The central state of a NUI application.
pub struct NUIRoot<T> {
    view: T,
    storage: Rc<Storage>,
}

impl<T> NUIRoot<T> {
    pub fn new(view: T) -> Self {
        Self {
            view,
            storage: Rc::new(Storage::new()),
        }
    }

    pub fn storage(&self) -> &Rc<Storage> {
        &self.storage
    }
}

impl<T> NUIRoot<T> where T: View {
    pub fn render(&mut self) -> Id<Primitive> {
        self.view.render(&Context::new(self.storage.clone()))
    }
}
