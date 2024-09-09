use nuit_derive::Bind;

use crate::{Access, Binding, Context, Event, EventResponse, Id, IdPath, IdentifyExt, Node, View};

/// A view that lets the user choose a value.
#[derive(Debug, Clone, Bind)]
pub struct Picker<C> {
    title: String,
    selection: Binding<Id>,
    content: C,
}

impl<C> Picker<C> {
    pub fn new(title: impl Into<String>, selection: Binding<Id>, content: C) -> Self {
        Self { title: title.into(), selection, content }
    }
}

impl<C> View for Picker<C> where C: View {
    fn fire(&self, event: &Event, event_path: &IdPath, _context: &Context) -> EventResponse {
        assert!(event_path.is_root());
        if let Event::UpdatePickerSelection { id } = event {
            self.selection.set(id.clone());
        }
        EventResponse::default()
    }

    fn render(&self, context: &Context) -> Node {
        Node::Picker {
            title: self.title.clone(),
            selection: self.selection.get(),
            content: Box::new(self.content.render(&context.child(0)).identify(0)),
        }
    }
}
