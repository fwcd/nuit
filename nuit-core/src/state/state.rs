use std::{cell::RefCell, rc::Rc};

use crate::{Animation, Binding, Storage};

use super::StateKey;

/// A wrapper around a value managed by nuit.
#[derive(Clone)]
pub struct State<T> {
    initial_value: T,
    storage: RefCell<Option<Rc<Storage>>>,
    key: RefCell<Option<StateKey>>,
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

    pub fn link(&self, storage: &Rc<Storage>, key: StateKey) {
        *self.storage.borrow_mut() = Some(storage.clone());
        *self.key.borrow_mut() = Some(key.clone());

        storage.initialize_if_needed(key, || self.initial_value.clone());
    }

    pub fn get(&self) -> T {
        let storage = self.storage.borrow();
        let storage = storage.as_ref().expect("Storage not linked prior to get");
        storage.get::<T>(self.key.borrow().as_ref().unwrap())
    }

    fn add_change(&self, value: impl Into<T>, animation: Option<Animation>) {
        let storage = self.storage.borrow();
        let storage = storage.as_ref().expect("Storage not linked prior to set");
        storage.add_change(self.key.borrow().clone().unwrap(), value.into(), animation);
    }

    pub fn set(&self, value: impl Into<T>) {
        self.add_change(value, None);
    }

    pub fn set_with(&self, animation: Animation, value: impl Into<T>) {
        self.add_change(value, Some(animation));
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
