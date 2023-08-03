use std::rc::Rc;

use crate::{Storage, IdPath, Binding};

#[derive(Clone)]
pub struct State<T> {
    initial_value: T,
    storage: Option<Rc<Storage>>,
    key: Option<(IdPath, usize)>,
}

impl<T> State<T> where T: 'static + Clone {
    pub fn new(initial_value: impl Into<T>) -> Self {
        Self {
            initial_value: initial_value.into(),
            storage: None,
            key: None,
        }
    }

    pub fn link(&mut self, storage: Rc<Storage>, id_path: IdPath, i: usize) {
        let key = (id_path.clone(), i);

        self.storage = Some(storage.clone());
        self.key = Some(key.clone());

        if !storage.contains_state(&key) {
            storage.insert_state(key, self.initial_value.clone());
        }
    }

    pub fn get(&self) -> T {
        let storage = self.storage.as_ref().expect("Storage not linked prior to get");
        storage.state::<T>(self.key.as_ref().unwrap())
    }

    pub fn set(&self, value: T) {
        let storage = self.storage.as_ref().expect("Storage not linked prior to set");
        storage.insert_state(self.key.clone().unwrap(), value);
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
