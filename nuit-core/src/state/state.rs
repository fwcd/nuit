use std::{cell::RefCell, rc::Rc};

use crate::{Access, Animation, Binding, Storage};

use super::StateKey;

/// A wrapper around a value managed by Nuit.
#[derive(Clone)]
pub struct State<T> {
    initial_value: T,
    storage: RefCell<Option<Rc<Storage>>>,
    key: RefCell<Option<StateKey>>,
}

impl<T> State<T> where T: 'static + Clone {
    /// Creates a new (unlinked) state from the given initial value.
    pub fn new(initial_value: impl Into<T>) -> Self {
        Self {
            initial_value: initial_value.into(),
            storage: RefCell::new(None),
            key: RefCell::new(None),
        }
    }

    /// Checks whether an underlying storage has been linked. May be useful for
    /// debugging.
    pub fn is_linked(&self) -> bool {
        self.storage.borrow().is_some() && self.key.borrow().is_some()
    }

    /// Links an underlying storage to this state. This is done automatically by
    /// the derived [`Bind::bind`] implementation, therefore this method should
    /// usually not be called manually.
    pub fn link(&self, storage: &Rc<Storage>, key: StateKey) {
        *self.storage.borrow_mut() = Some(storage.clone());
        *self.key.borrow_mut() = Some(key.clone());

        storage.initialize_if_needed(key, || self.initial_value.clone());
    }
}

impl<T> Access for State<T> where T: 'static + Clone {
    type Value = T;

    /// Fetches the underlying value.
    /// 
    /// # Panics
    /// 
    /// This will panic if the storage has not been linked first. Linking is
    /// performed by (the usually derived) [`Bind::bind`] implementation before
    /// [`View::render`] is called, so calling this method in the render
    /// implementation is generally safe (and encouraged).
    fn get(&self) -> T {
        let storage = self.storage.borrow();
        let storage = storage.as_ref().expect("Storage not linked before calling State::get");
        storage.get::<T>(self.key.borrow().as_ref().unwrap())
    }

    /// Changes the underlying value.
    /// 
    /// # Panics
    /// 
    /// This will panic if the storage has not been linked first. Linking is
    /// performed by (the usually derived) [`Bind::bind`] implementation before
    /// [`View::render`] is called, so calling this method in the render
    /// implementation is generally safe (and encouraged).
    fn change(&self, value: impl Into<T>, animation: Option<Animation>) {
        let storage = self.storage.borrow();
        let storage = storage.as_ref().expect("Storage not linked before calling State::change");
        storage.add_change(self.key.borrow().clone().unwrap(), value.into(), animation);
    }

    /// Obtains a [`Binding`] to the underlying value.
    /// 
    /// # Panics
    /// 
    /// This will panic if the storage has not been linked first. Linking is
    /// performed by (the usually derived) [`Bind::bind`] implementation before
    /// [`View::render`] is called, so calling this method in the render
    /// implementation is generally safe (and encouraged).
    fn binding(&self) -> Binding<T> {
        assert!(self.is_linked(), "Storage not linked before calling State::binding");
        Binding::from_access(self.clone())
    }
}

impl<T> Default for State<T> where T: 'static + Default + Clone {
    fn default() -> Self {
        Self::new(T::default())
    }
}
