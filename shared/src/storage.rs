use std::{collections::HashMap, any::Any, cell::RefCell};

use crate::IdPath;

// TODO: Use trees to model these id path keys (this should also allow us to take them by ref and delete subtrees easily)

pub struct Storage {
    state: RefCell<HashMap<(IdPath, usize), Box<dyn Any>>>,
    click_actions: RefCell<HashMap<IdPath, Box<dyn Fn()>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(HashMap::new()),
            click_actions: RefCell::new(HashMap::new()),
        }
    }

    pub fn contains_state(&self, key: &(IdPath, usize)) -> bool {
        self.state.borrow().contains_key(&key)
    }

    pub fn insert_state(&self, key: (IdPath, usize), value: impl Any) {
        self.state.borrow_mut().insert(key, Box::new(value));
    }

    pub fn state<T>(&self, key: &(IdPath, usize)) -> T where T: Clone + 'static {
        let state = &self.state.borrow()[key];
        state.downcast_ref::<T>().expect("State has invalid type").clone()
    }

    pub fn insert_click_action(&self, key: IdPath, action: impl Fn() + 'static) {
        self.click_actions.borrow_mut().insert(key, Box::new(action));
    }

    pub fn fire_click_action(&self, key: &IdPath) {
        if let Some(action) = self.click_actions.borrow().get(key) {
            action();
        }
    }
}
