use crate::{View, Node, Bind, Context, Identified};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If<T, F> {
    then_view: Option<T>,
    else_view: Option<F>,
}

impl<T> If<T, ()> {
    pub fn new(condition: bool, then_view: impl FnOnce() -> T) -> Self {
        Self {
            then_view: if condition { Some(then_view()) } else { None },
            else_view: None,
        }
    }
}

impl<T, F> If<T, F> {
    pub fn new_or_else(condition: bool, then_view: impl FnOnce() -> T, else_view: impl FnOnce() -> F) -> Self {
        Self {
            then_view: if condition { Some(then_view()) } else { None },
            else_view: if !condition { Some(else_view()) } else { None },
        }
    }
}

impl<T, F> Bind for If<T, F> where T: Bind, F: Bind {}

impl<T, F> View for If<T, F> where T: View, F: View {
    fn render(&mut self, context: &Context) -> Identified<Node> {
        if let Some(ref mut then_view) = self.then_view {
            then_view.render(&context.child(0))
        } else if let Some(ref mut else_view) = self.else_view {
            else_view.render(&context.child(1))
        } else {
            context.identify(Node::Empty)
        }
    }
}
