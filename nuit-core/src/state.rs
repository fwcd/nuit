use std::{cell::RefCell, rc::Rc};

use crate::{Storage, IdPath, Binding, IdPathBuf};

/// A wrapper around a value managed by nuit.
#[derive(Clone)]
pub struct State<T> {
    initial_value: T,
    storage: RefCell<Option<Rc<Storage>>>,
    key: RefCell<Option<(IdPathBuf, usize)>>,
}

impl<T> State<T> where T: 'static + Clone {
    pub fn new(initial_value: impl Into<T>) -> Self {
        Self {
            initial_value: initial_value.into(),
            storage: RefCell::new(None),
            key: RefCell::new(None),
        }
    }

    pub fn is_linked(&self) -> bool {
        self.storage.borrow().is_some() && self.key.borrow().is_some()
    }

    pub fn link(&self, storage: Rc<Storage>, id_path: &IdPath, i: usize) {
        let key = (id_path.to_owned(), i);

        *self.storage.borrow_mut() = Some(storage.clone());
        *self.key.borrow_mut() = Some(key.clone());

        storage.initialize_if_needed(key, || self.initial_value.clone());
    }

    pub fn get(&self) -> T {
        let storage = self.storage.borrow();
        let storage = storage.as_ref().expect("Storage not linked prior to get");
        storage.get::<T>(self.key.borrow().as_ref().unwrap())
    }

    pub fn set(&self, value: impl Into<T>) {
        let storage = self.storage.borrow();
        let storage = storage.as_ref().expect("Storage not linked prior to set");
        storage.add_change(self.key.borrow().clone().unwrap(), value.into());
    }

    pub fn binding(&self) -> Binding<T> {
        let self1 = self.clone();
        let self2 = self.clone();
        Binding::new(
            move || self1.get(),
            move |value| self2.set(value),
        )
    }
}

impl<T> Default for State<T> where T: 'static + Default + Clone {
    fn default() -> Self {
        Self::new(T::default())
    }
}
