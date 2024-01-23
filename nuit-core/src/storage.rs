use std::{collections::HashMap, any::Any, cell::RefCell};

use crate::IdPathBuf;

// TODO: Use trees to model these id path keys (this should also allow us to take them by ref and delete subtrees easily)

pub struct Storage {
    state: RefCell<HashMap<(IdPathBuf, usize), Box<dyn Any>>>,
    changes: RefCell<HashMap<(IdPathBuf, usize), Box<dyn Any>>>,
    update_callback: RefCell<Option<Box<dyn Fn()>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(HashMap::new()),
            changes: RefCell::new(HashMap::new()),
            update_callback: RefCell::new(None),
        }
    }

    pub(crate) fn initialize_if_needed<V>(&self, key: (IdPathBuf, usize), value: impl FnOnce() -> V) where V: 'static {
        if !self.state.borrow().contains_key(&key) {
            self.state.borrow_mut().insert(key, Box::new(value()));
        }
    }

    pub(crate) fn add_change<V>(&self, key: (IdPathBuf, usize), value: V) where V: 'static {
        self.changes.borrow_mut().insert(key, Box::new(value));
        self.fire_update_callback();
    }

    pub(crate) fn get<T>(&self, key: &(IdPathBuf, usize)) -> T where T: Clone + 'static {
        let state = &self.state.borrow()[key];
        state.downcast_ref::<T>().expect("State has invalid type").clone()
    }

    pub(crate) fn apply_changes(&self) {
        let mut state = self.state.borrow_mut();
        for (key, value) in self.changes.borrow_mut().drain() {
            state.insert(key, value);
        }
    }

    fn fire_update_callback(&self) {
        if let Some(update_callback) = self.update_callback.borrow().as_ref() {
            update_callback();
        }
    }

    pub fn set_update_callback(&self, update_callback: impl Fn() + 'static) {
        *self.update_callback.borrow_mut() = Some(Box::new(update_callback));
    }
}
