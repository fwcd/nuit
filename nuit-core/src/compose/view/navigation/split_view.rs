use nuit_derive::Bind;
use crate::{Context, Event, EventResponse, Id, IdPath, IdentifyExt, Node, View};

#[derive(Debug, Bind)]
pub struct NavigationSplitView<S, C, D> {
    sidebar: S,
    content: C,
    detail: D,
}

impl<S, C, D> NavigationSplitView<S, C, D> {
    #[must_use]
    pub const fn new(sidebar: S, content: C, detail: D) -> Self {
        Self {
            sidebar,
            content,
            detail,
        }
    }

    pub fn with_content<C2>(self, content: C2) -> NavigationSplitView<S, C2, D> {
        NavigationSplitView {
            sidebar: self.sidebar,
            content,
            detail: self.detail,
        }
    }

    pub fn with_detail<D2>(self, detail: D2) -> NavigationSplitView<S, C, D2> {
        NavigationSplitView {
            sidebar: self.sidebar,
            content: self.content,
            detail,
        }
    }
}

impl<S> NavigationSplitView<S, (), ()> {
    #[must_use]
    pub const fn with_sidebar(sidebar: S) -> Self {
        Self {
            sidebar,
            content: (),
            detail: (),
        }
    }
}

impl<T> From<T> for NavigationSplitView<T, (), ()> {
    fn from(wrapped: T) -> Self {
        Self::with_sidebar(wrapped)
    }
}

impl<S, C, D> View for NavigationSplitView<S, C, D> where S: View, C: View, D: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) -> EventResponse {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.sidebar.fire(event, event_path.tail(), &context.child(0)),
                Id::Index(1) => self.content.fire(event, event_path.tail(), &context.child(1)),
                Id::Index(2) => self.detail.fire(event, event_path.tail(), &context.child(2)),
                i => panic!("Cannot fire event for child id {i} on NavigationSplitView which only has three children"),
            }
        } else {
            EventResponse::default()
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::NavigationSplitView {
            sidebar: Box::new(self.sidebar.render(&context.child(0)).identify(0)),
            content: Box::new(self.content.render(&context.child(1)).identify(1)),
            detail: Box::new(self.detail.render(&context.child(2)).identify(2)),
        }
    }
}
