use crate::{View, Node, Bind, Context, Identified, modifier::Modifier, IdPath, Event, Id};

#[derive(Debug, Clone, PartialEq)]
pub struct Modified<T> {
    wrapped: T,
    modifier: Modifier,
}

impl<T> Modified<T> {
    pub fn new(wrapped: T, modifier: Modifier) -> Self {
        Self {
            wrapped,
            modifier,
        }
    }
}

impl<T> Bind for Modified<T> where T: Bind {}

impl<T> View for Modified<T> where T: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, &id_path.tail()),
                i => panic!("Cannot fire event for child id {} on Modified which only has one child", i)
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Modified {
            wrapped: Box::new(self.wrapped.render(&context.child(0))),
            modifier: self.modifier,
        })
    }
}
