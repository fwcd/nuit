use std::{cell::RefCell, rc::Rc};

use crate::{Context, Diff, Event, IdPath, IdPathBuf, Node, Storage, View};

/// The central state of a Nuit application.
pub struct Root<T> {
    view: RefCell<T>,
    storage: Rc<Storage>,
    last_render: RefCell<Node>,
}

impl<T> Root<T> {
    pub fn new(view: T) -> Self {
        Self {
            view: RefCell::new(view),
            storage: Rc::new(Storage::new()),
            last_render: RefCell::new(Node::Empty {}),
        }
    }

    pub fn storage(&self) -> &Rc<Storage> {
        &self.storage
    }
}

impl<T> Root<T> where T: View {
    pub fn render(&self) -> Node {
        let context = Context::new(self.storage.clone());
        let new_render = self.storage.with_preapplied_changes(|| {
            self.view.borrow().render(&context)
        });

        {
            let last_render = self.last_render.borrow();
            let diff = new_render.diff(&last_render);

            for (id_path, _) in &diff.removed {
                self.view.borrow().fire(&Event::Disappear, id_path, &context);
            }

            self.storage.apply_changes();

            for (id_path, _) in &diff.added {
                self.view.borrow().fire(&Event::Appear, id_path, &context);
            }
        }

        *self.last_render.borrow_mut() = new_render.clone();
        new_render
    }

    pub fn render_json(&self) -> String {
        let node = self.render();
        serde_json::to_string(&node).expect("Could not serialize view")
    }

    pub fn fire_event_json(&self, id_path_json: &str, event_json: &str) {
        let id_path: IdPathBuf = serde_json::from_str(id_path_json).expect("Could not deserialize id path");
        let event: Event = serde_json::from_str(event_json).expect("Could not deserialize event");
        self.fire_event(&id_path, &event);
    }

    pub fn fire_event(&self, id_path: &IdPath, event: &Event) {
        self.view.borrow().fire(event, id_path, &Context::new(self.storage.clone()));
    }

    pub fn set_update_callback(&self, update_callback: impl Fn() + 'static) {
        self.storage.set_update_callback(update_callback);
    }
}
