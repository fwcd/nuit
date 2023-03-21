use std::{collections::HashMap, any::Any, cell::RefCell};

use crate::{IdPath, Event};

// TODO: Use trees to model these id path keys (this should also allow us to take them by ref and delete subtrees easily)

pub struct Storage {
    state: RefCell<HashMap<(IdPath, usize), Box<dyn Any>>>,
    event_handlers: RefCell<HashMap<IdPath, Box<dyn Fn(Event)>>>,
    update_callback: RefCell<Option<Box<dyn Fn()>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            state: RefCell::new(HashMap::new()),
            event_handlers: RefCell::new(HashMap::new()),
            update_callback: RefCell::new(None),
        }
    }

    pub fn contains_state(&self, key: &(IdPath, usize)) -> bool {
        self.state.borrow().contains_key(&key)
    }

    pub fn insert_state(&self, key: (IdPath, usize), value: impl Any) {
        self.state.borrow_mut().insert(key, Box::new(value));
        self.fire_update_callback();
    }

    pub fn state<T>(&self, key: &(IdPath, usize)) -> T where T: Clone + 'static {
        let state = &self.state.borrow()[key];
        state.downcast_ref::<T>().expect("State has invalid type").clone()
    }

    pub fn insert_event_handler(&self, key: IdPath, handler: impl Fn(Event) + 'static) {
        self.event_handlers.borrow_mut().insert(key, Box::new(handler));
    }

    pub fn fire_event(&self, key: &IdPath, event: Event) {
        if let Some(handler) = self.event_handlers.borrow().get(key) {
            handler(event);
            self.fire_update_callback();
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
