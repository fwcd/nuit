use nuit_derive::Bind;

use crate::{Access, Binding, Context, Event, EventResponse, IdPath, Node, View};

/// A user-modifiable text field.
#[derive(Debug, Clone, Bind)]
pub struct TextField {
    content: Binding<String>,
}

impl TextField {
    #[must_use]
    pub const fn new(content: Binding<String>) -> Self {
        Self { content }
    }
}

impl View for TextField {
    fn fire(&self, event: &Event, event_path: &IdPath, _context: &Context) -> EventResponse {
        assert!(event_path.is_root());
        if let Event::UpdateText { content } = event {
            self.content.set(content.to_owned());
        }
        EventResponse::default()
    }

    fn render(&self, _context: &Context) -> Node {
        Node::TextField { content: self.content.get() }
    }
}
