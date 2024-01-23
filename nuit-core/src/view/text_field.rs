use crate::{View, Node, Bind, Context, Identified, Binding, Event, IdPath};

#[derive(Debug, Clone)]
pub struct TextField {
    content: Binding<String>,
}

impl TextField {
    pub fn new(content: Binding<String>) -> Self {
        Self { content }
    }
}

impl Bind for TextField {
    fn bind(&mut self, _context: &Context) {}
}

impl View for TextField {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        assert!(id_path.is_root());
        if let Event::UpdateText { content } = event {
            self.content.set(content.to_owned());
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        self.bind(context);
        context.identify(Node::TextField { content: self.content.get() })
    }
}
