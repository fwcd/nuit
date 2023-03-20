use std::{rc::Rc, cell::RefCell};

use crate::{Storage, IdPath};

pub struct Context {
    id_path: IdPath,
    storage: Rc<RefCell<Storage>>,
}

impl Context {
    pub fn new(storage: Rc<RefCell<Storage>>) -> Self {
        Self {
            id_path: IdPath::root(),
            storage,
        }
    }

    pub fn storage(&self) -> &Rc<RefCell<Storage>> {
        &self.storage
    }

    pub fn id_path(&self) -> &IdPath {
        &self.id_path
    }

    pub fn child(&self, i: usize) -> Self {
        Self {
            id_path: self.id_path.child(i),
            storage: self.storage.clone(),
        }
    }
}
