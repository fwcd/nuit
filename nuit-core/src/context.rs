use std::rc::Rc;

use crate::{Storage, IdPath, Id, IdPathBuf};

/// A context used during rendering that tracks the path to the current view and
/// holds a reference to internal storage.
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

    #[must_use]
    pub fn storage(&self) -> &Rc<Storage> {
        &self.storage
    }

    #[must_use]
    pub fn id_path(&self) -> &IdPath {
        &self.id_path
    }

    #[must_use]
    pub fn child(&self, id: impl Into<Id>) -> Self {
        Self {
            id_path: self.id_path.child(id),
            storage: self.storage.clone(),
        }
    }
}
