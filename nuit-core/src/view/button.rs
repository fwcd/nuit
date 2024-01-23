use crate::{View, Node, Bind, Context, Event, IdPath, Id, IdentifyExt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button<T, F> {
    label: T,
    action: Option<F>,
}

impl<T, F> Button<T, F> {
    pub fn new(label: T, action: F) -> Self {
        Self {
            label,
            action: Some(action),
        }
    }
}

impl<T, F> Bind for Button<T, F> where T: Bind, F: Fn() + 'static {}

impl<T, F> View for Button<T, F> where T: View, F: Fn() + 'static {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.label.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on Button which only has one child", i)
            }
        } else if let Event::Click {} = event {
            if let Some(ref action) = self.action {
                action();
            }
        }
    }

    fn render(&mut self, context: &Context) -> Node {
        Node::Button { label: Box::new(self.label.render(&context.child(0)).identify(0)) }
    }
}
