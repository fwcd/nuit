use std::{collections::HashMap, any::Any, cell::{RefCell, Cell}};

use super::StateKey;

/// A facility that manages view state internally.
pub struct Storage {
    state: RefCell<HashMap<StateKey, Box<dyn Any>>>,
    changes: RefCell<HashMap<StateKey, Box<dyn Any>>>,
    preapply: Cell<bool>,
    update_callback: RefCell<Option<Box<dyn Fn()>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(HashMap::new()),
            changes: RefCell::new(HashMap::new()),
            preapply: Cell::new(false),
            update_callback: RefCell::new(None),
        }
    }

    pub(crate) fn initialize_if_needed<V>(&self, key: StateKey, value: impl FnOnce() -> V) where V: 'static {
        if !self.state.borrow().contains_key(&key) {
            self.state.borrow_mut().insert(key, Box::new(value()));
        }
    }

    pub(crate) fn add_change<V>(&self, key: StateKey, value: V) where V: 'static {
        self.changes.borrow_mut().insert(key, Box::new(value));
        self.fire_update_callback();
    }

    pub(crate) fn get<T>(&self, key: &StateKey) -> T where T: Clone + 'static {
        if self.preapply.get() && let Some(changed) = self.changes.borrow().get(key) {
            changed.downcast_ref::<T>().cloned()
        } else {
            self.state.borrow()[key].downcast_ref::<T>().cloned()
        }
        .expect("State has invalid type")
    }

    pub(crate) fn with_preapplied_changes<T>(&self, action: impl FnOnce() -> T) -> T {
        self.preapply.set(true);
        let result = action();
        self.preapply.set(false);
        result
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
