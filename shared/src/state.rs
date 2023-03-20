use std::{rc::Rc, cell::RefCell};

use crate::{Storage, IdPath};

#[derive(Clone)]
pub struct State<T> {
    initial_value: T,
    storage: Option<Rc<RefCell<Storage>>>,
    key: Option<(IdPath, usize)>,
}

impl<T> State<T> where T: 'static + Clone {
    pub fn new(initial_value: T) -> Self {
        Self {
            initial_value,
            storage: None,
            key: None,
        }
    }

    pub fn link(&mut self, storage: Rc<RefCell<Storage>>, id_path: IdPath, i: usize) {
        let key = (id_path.clone(), i);

        self.storage = Some(storage.clone());
        self.key = Some(key.clone());
        let mut storage = storage.borrow_mut();

        if !storage.contains_state(&key) {
            storage.insert_state(key, self.initial_value.clone());
        }
    }

    pub fn get(&self) -> T {
        let storage = self.storage.as_ref().expect("Storage not linked prior to get").borrow();
        storage.state::<T>(self.key.as_ref().unwrap()).clone()
    }

    pub fn set(&self, value: T) {
        let mut storage = self.storage.as_ref().expect("Storage not linked prior to set").borrow_mut();
        *storage.state_mut::<T>(self.key.as_ref().unwrap()) = value;
    }
}
