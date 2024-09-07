use nuit_derive::Bind;

use crate::{Context, Event, HasId, IdPath, Identified, IdentifyExt, Node, View};

/// A group of views that is dynamically computed from a given collection.
#[derive(Debug, Clone, PartialEq, Eq, Bind)]
pub struct ForEach<V> {
    children: Vec<Identified<V>>,
}

// Note: We need the trait bounds here already since the collection types may
// otherwise get inferred incorrectly, resulting in potentially obscure errors
// where the composite type appears not to implement View.

impl<V> ForEach<V> where V: View {
    /// Creates new views from the given collection.
    pub fn new<I: HasId>(collection: impl IntoIterator<Item = I>, view_func: impl Fn(I) -> V) -> Self {
        Self {
            children: collection
                .into_iter()
                .map(|item| {
                    let id = item.id();
                    view_func(item).identify(id)
                })
                .collect()
        }
    }

    /// Creates new views from the given collection, additionally taking the index.
    pub fn with_index<I: HasId>(collection: impl IntoIterator<Item = I>, view_func: impl Fn(usize, I) -> V) -> Self {
        Self {
            children: collection
                .into_iter()
                .enumerate()
                .map(|(i, item)| {
                    let id = item.id();
                    view_func(i, item).identify(id)
                })
                .collect()
        }
    }

    /// Creates new views from the given collection, identified by their index.
    pub fn with_index_id<I>(collection: impl IntoIterator<Item = I>, view_func: impl Fn(usize, I) -> V) -> Self {
        Self {
            children: collection
                .into_iter()
                .enumerate()
                .map(|(i, item)| view_func(i, item).identify(i))
                .collect()
        }
    }
}

// TODO: Figure out if we can write the bound on references to avoid the clone

impl<V> View for ForEach<V> where V: View {
    fn fire(&self, event: &Event, event_path: &IdPath, context: &Context) {
        if let Some(head) = event_path.head() {
            if let Some(view) = self.children.iter().find(|view| view.id() == &head) {
                view.value().fire(event, event_path.tail(), &context.child(view.id().clone()));
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::Group {
            children: self.children
                .iter()
                .map(|view| view.as_ref().map_with_id(|id, v| v.render(&context.child(id.clone()))))
                .collect(),
        }
    }
}
