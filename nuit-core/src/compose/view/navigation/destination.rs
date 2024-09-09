use std::marker::PhantomData;

use nuit_derive::Bind;
use serde::de::DeserializeOwned;

use crate::{Context, Event, EventResponse, Id, IdPath, IdentifyExt, Node, View};

#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct NavigationDestination<T, F, V, D> {
    wrapped: T,
    destination_func: F,
    phantom_value: PhantomData<V>,
    phantom_destination_view: PhantomData<D>,
}

impl<T, F, V, D> NavigationDestination<T, F, V, D> {
    #[must_use]
    pub const fn new(wrapped: T, destination_func: F) -> Self {
        Self {
            wrapped,
            destination_func,
            phantom_value: PhantomData,
            phantom_destination_view: PhantomData,
        }
    }
}

impl<T, F, V, D> View for NavigationDestination<T, F, V, D>
where
    T: View,
    F: Fn(V) -> D,
    V: DeserializeOwned,
    D: View,
{
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) -> EventResponse {
        if let Some(head) = event_path.head() {
            match head {
                Id::Index(0) => self.wrapped.fire(event, event_path.tail(), &context.child(0)),
                Id::Index(i) => panic!("Cannot fire event for child id {i} on NavigationDestination"),
                Id::String(value_json) => {
                    let value = serde_json::from_str(&value_json).expect("Could not deserialize navigation destination id");
                    let destination = (self.destination_func)(value);
                    destination.fire(event, event_path.tail(), &context.child(value_json))
                },
            }
        } else if let Event::GetNavigationDestination { value } = event {
            let value_json = serde_json::to_string(value).expect("Could not serialize navigation destination id");
            let id = Id::string(value_json);

            let value = serde_json::from_value(value.clone()).expect("Could not deserialize navigation destination value");
            let destination = (self.destination_func)(value);

            let node = destination.render(&context.child(id.clone())).identify(id);
            EventResponse::Node { node }
        } else {
            EventResponse::default()
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::NavigationDestination {
            wrapped: Box::new(self.wrapped.render(&context.child(0)).identify(0)),
        }
    }
}
