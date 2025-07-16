use nuit_derive::Bind;

use crate::{Access, Binding, Context, Event, EventResponse, IdPath, Node, View};

/// A control that toggles between on and off states.
#[derive(Debug, Clone, Bind)]
pub struct Toggle {
    is_on: Binding<bool>,
}

impl Toggle {
    /// Creates a new Toggle with the given binding.
    #[must_use]
    pub fn new(is_on: Binding<bool>) -> Self {
        Self { is_on }
    }
}

impl View for Toggle {
    fn fire(&self, event: &Event, event_path: &IdPath, _context: &Context) -> EventResponse {
        assert!(event_path.is_root());
        if let Event::ToggleChange {} = event {
            let current = self.is_on.get();
            self.is_on.set(!current);
        }
        EventResponse::default()
    }

    fn render(&self, _context: &Context) -> Node {
        Node::Toggle { is_on: self.is_on.get() }
    }
}