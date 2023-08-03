use std::rc::Rc;

use crate::{Storage, Primitive, View, Context, Id, Event, IdPath};

/// The central state of a Nuit application.
pub struct Root<T> {
    view: T,
    storage: Rc<Storage>,
}

impl<T> Root<T> {
    pub fn new(view: T) -> Self {
        Self {
            view,
            storage: Rc::new(Storage::new()),
        }
    }

    pub fn storage(&self) -> &Rc<Storage> {
        &self.storage
    }
}

impl<T> Root<T> where T: View {
    pub fn render(&mut self) -> Id<Primitive> {
        self.view.render(&Context::new(self.storage.clone()))
    }

    pub fn render_json(&mut self) -> String {
        let primitive = self.render();
        serde_json::to_string(&primitive).expect("Could not serialize view")
    }

    pub fn fire_event_json(&mut self, id_path_json: &str, event_json: &str) {
        let id_path: IdPath = serde_json::from_str(id_path_json).expect("Could not deserialize id path");
        let event: Event = serde_json::from_str(event_json).expect("Could not deserialize event");
        self.storage.fire_event(&id_path, event);
    }

    pub fn set_update_callback(&mut self, update_callback: impl Fn() + 'static) {
        self.storage.set_update_callback(update_callback);
    }
}
