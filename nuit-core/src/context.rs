use std::rc::Rc;

use crate::{Storage, IdPath, Id};

pub struct Context {
    id_path: IdPath,
    storage: Rc<Storage>,
}

impl Context {
    pub fn new(storage: Rc<Storage>) -> Self {
        Self {
            id_path: IdPath::root(),
            storage,
        }
    }

    pub fn storage(&self) -> &Rc<Storage> {
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

    pub fn identify<T>(&self, value: T) -> Id<T> {
        Id::new(self.id_path.clone(), value)
    }
}
