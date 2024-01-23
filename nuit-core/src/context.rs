use std::rc::Rc;

use crate::{Storage, IdPath, Identified, Id, IdPathBuf};

pub struct Context {
    id_path: IdPathBuf,
    storage: Rc<Storage>,
}

impl Context {
    pub fn new(storage: Rc<Storage>) -> Self {
        Self {
            id_path: IdPathBuf::root(),
            storage,
        }
    }

    pub fn storage(&self) -> &Rc<Storage> {
        &self.storage
    }

    pub fn id_path(&self) -> &IdPath {
        &self.id_path
    }

    pub fn child(&self, id: impl Into<Id>) -> Self {
        Self {
            id_path: self.id_path.child(id),
            storage: self.storage.clone(),
        }
    }

    pub fn identify<T>(&self, value: T) -> Identified<T> {
        Identified::new(&self.id_path, value)
    }
}
