use crate::{Bind, Context, Event, HasId, IdPath, Identified, IdentifyExt, Node, View};

/// A group of views that is dynamically computed from a given collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForEach<V> {
    children: Vec<Identified<V>>,
}

// Note: We need the trait bounds here already since the collection types may
// otherwise get inferred incorrectly, resulting in potentially obscure errors
// where the composite type appears not to implement View.

impl<V> ForEach<V> where V: Bind {
    pub fn new<I: HasId>(collection: impl IntoIterator<Item = I>, view_func: impl Fn(I) -> V) -> Self {
        Self {
            children: collection
                .into_iter()
                .map(|i| {
                    let id = i.id();
                    view_func(i).identify(id)
                })
                .collect()
        }
    }
}

// TODO: Figure out if we can write the bound on references to avoid the clone

impl<V> Bind for ForEach<V> where V: Bind {}

impl<V> View for ForEach<V> where V: View {
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            if let Some(view) = self.children.iter().find(|view| view.id() == &head) {
                view.value().fire(event, &id_path.tail());
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
