use crate::{View, Node, Bind, Context, Binding, Event, IdPath, Id, IdentifyExt};

#[derive(Debug, Clone)]
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

impl<C> Bind for Picker<C> where C: Bind {}

impl<C> View for Picker<C> where C: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        assert!(id_path.is_root());
        if let Event::UpdatePickerSelection { id } = event {
            self.selection.set(id.clone());
        }
    }

    fn render(&mut self, context: &Context) -> Node {
        Node::Picker {
            title: self.title.clone(),
            selection: self.selection.get(),
            content: Box::new(self.content.render(&context.child(0)).identify(0)),
        }
    }
}
