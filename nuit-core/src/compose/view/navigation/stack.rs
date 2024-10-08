use nuit_derive::Bind;
use serde::{de::DeserializeOwned, Serialize};

use crate::{Access, Binding, Context, Event, EventResponse, Id, IdPath, IdentifyExt, Node, View};

#[derive(Debug, Clone, Bind)]
pub struct NavigationStack<T, I> {
    path: Option<Binding<Vec<I>>>,
    wrapped: T,
}

impl<T, I> NavigationStack<T, I> {
    pub const fn new(path: Option<Binding<Vec<I>>>, wrapped: T) -> Self {
        Self {
            path,
            wrapped,
        }
    }

    pub const fn with_path(path: Binding<Vec<I>>, wrapped: T) -> Self {
        Self {
            path: Some(path),
            wrapped,
        }
    }
}

impl<T> From<T> for NavigationStack<T, ()> {
    fn from(wrapped: T) -> Self {
        Self::new(None, wrapped)
    }
}

impl<T, I> View for NavigationStack<T, I> where T: View, I: Serialize + DeserializeOwned + 'static {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) -> EventResponse {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                i => panic!("Cannot fire event for child id {i} on NavigationStack which only has one child"),
            }
        } else {
            if let Event::UpdateNavigationPath { path } = event {
                if let Some(path_binding) = &self.path {
                    let path = path.iter()
                        .map(|item| serde_json::from_value(item.clone()))
                        .collect::<Result<Vec<I>, _>>()
                        .expect("Could not deserialize navigation path");
                    path_binding.set(path);
                } else {
                    eprintln!("Warning: Ignoring navigation path update since no path binding it set.");
                }
            }
            EventResponse::default()
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::NavigationStack {
            path: self.path.as_ref().map(|path|
                path.get()
                    .iter()
                    .map(serde_json::to_value)
                    .collect::<Result<Vec<_>, _>>()
                    .expect("Could not serialize navigation path")
            ),
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0))
        }
    }
}
