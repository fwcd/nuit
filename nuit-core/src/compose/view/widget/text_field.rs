use nuit_derive::Bind;

use crate::{View, Node, Context, Binding, Event, IdPath};

/// A user-modifiable text field.
#[derive(Debug, Clone, Bind)]
pub struct TextField {
    content: Binding<String>,
}

impl TextField {
    #[must_use]
    pub fn new(content: Binding<String>) -> Self {
        Self { content }
    }
}

impl View for TextField {
    fn fire(&self, event: &Event, event_path: &IdPath, _context: &Context) {
        assert!(event_path.is_root());
        if let Event::UpdateText { content } = event {
            self.content.set(content.to_owned());
        }
    }

    fn render(&self, _context: &Context) -> Node {
        Node::TextField { content: self.content.get() }
    }
}
