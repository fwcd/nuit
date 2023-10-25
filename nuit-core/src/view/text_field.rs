use crate::{View, Node, Bind, Context, Id, Binding, Event};

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
    fn bind(&mut self, context: &Context) {
        let storage = context.storage();
        let content = self.content.clone();
        storage.insert_event_handler(context.id_path().clone(), move |event| {
            if let Event::UpdateText { content: new_content } = event {
                content.set(new_content);
            }
        });
    }
}

impl View for TextField {
    fn render(&mut self, context: &Context) -> Id<Node> {
        self.bind(context);
        context.identify(Node::TextField { content: self.content.get() })
    }
}
