use std::{collections::HashMap, any::Any};

use crate::IdPath;

// TODO: Use trees to model these id path keys (this should also allow us to take them by ref and delete subtrees easily)

pub struct Storage {
    state: HashMap<(IdPath, usize), Box<dyn Any>>,
    click_actions: HashMap<IdPath, Box<dyn Fn()>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            state: HashMap::new(),
            click_actions: HashMap::new(),
        }
    }

    pub fn contains_state(&self, key: &(IdPath, usize)) -> bool {
        self.state.contains_key(&key)
    }

    pub fn insert_state(&mut self, key: (IdPath, usize), value: impl Any) {
        self.state.insert(key, Box::new(value));
    }

    pub fn state<T>(&self, key: &(IdPath, usize)) -> &T where T: 'static {
        let state = &self.state[key];
        state.downcast_ref::<T>().expect("State has invalid type")
    }

    pub fn state_mut<T>(&mut self, key: &(IdPath, usize)) -> &mut T where T: 'static {
        let state = self.state.get_mut(key).unwrap();
        state.downcast_mut::<T>().expect("State has invalid type")
    }

    pub fn insert_click_action(&mut self, key: IdPath, action: impl Fn() + 'static) {
        self.click_actions.insert(key, Box::new(action));
    }

    pub fn fire_click_action(&self, key: &IdPath) {
        if let Some(action) = self.click_actions.get(key) {
            action();
        }
    }
}
