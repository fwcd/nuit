use std::rc::Rc;

use crate::{Context, Event, IdPath, IdPathBuf, Node, NodeDiff, Storage, View};

/// The central state of a Nuit application.
pub struct Root<T> {
    view: T,
    storage: Rc<Storage>,
    last_render: Node,
}

impl<T> Root<T> {
    pub fn new(view: T) -> Self {
        Self {
            view,
            storage: Rc::new(Storage::new()),
            last_render: Node::Empty {},
        }
    }

    pub fn storage(&self) -> &Rc<Storage> {
        &self.storage
    }
}

impl<T> Root<T> where T: View {
    pub fn render(&mut self) -> Node {
        let new_render = self.storage.with_preapplied_changes(|| {
            self.view.render(&Context::new(self.storage.clone()))
        });

        let diff = NodeDiff::between(&new_render, &self.last_render);

        for id_path in diff.removed() {
            self.view.fire(&Event::Disappear, id_path)
        }

        self.storage.apply_changes();

        for id_path in diff.added() {
            self.view.fire(&Event::Appear, id_path)
        }

        self.last_render = new_render.clone();
        new_render
    }

    pub fn render_json(&mut self) -> String {
        let node = self.render();
        serde_json::to_string(&node).expect("Could not serialize view")
    }

    pub fn fire_event_json(&mut self, id_path_json: &str, event_json: &str) {
        let id_path: IdPathBuf = serde_json::from_str(id_path_json).expect("Could not deserialize id path");
        let event: Event = serde_json::from_str(event_json).expect("Could not deserialize event");
        self.fire_event(&id_path, &event);
    }

    pub fn fire_event(&mut self, id_path: &IdPath, event: &Event) {
        self.view.fire(event, id_path);
    }

    pub fn set_update_callback(&mut self, update_callback: impl Fn() + 'static) {
        self.storage.set_update_callback(update_callback);
    }
}
