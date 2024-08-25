use std::marker::PhantomData;

use crate::{View, Node, Bind, Context, HasId, Event, IdPath, IdentifyExt};

/// A group of views that is dynamically computed from a given collection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForEach<C, I, F, V> {
    collection: C,
    view_func: F,
    phantom_item: PhantomData<I>,
    phantom_view: PhantomData<V>,
}

// Note: We need the trait bounds here already since the collection types may
// otherwise get inferred incorrectly, resulting in potentially obscure errors
// where the composite type appears not to implement View.

impl<C, I, F, V> ForEach<C, I, F, V>
where
    for<'a> &'a C: IntoIterator<Item = &'a I> + Clone,
    I: HasId,
    F: Fn(&I) -> V,
    V: Bind
{
    pub fn new(collection: C, view_func: F) -> Self {
        Self {
            collection,
            view_func,
            phantom_item: PhantomData,
            phantom_view: PhantomData,
        }
    }
}

// TODO: Figure out if we can write the bound on references to avoid the clone

impl<C, I, F, V> Bind for ForEach<C, I, F, V>
where
    for<'a> &'a C: IntoIterator<Item = &'a I> + Clone,
    I: HasId,
    F: Fn(&I) -> V,
    V: Bind
{}

impl<C, I, F, V> View for ForEach<C, I, F, V>
where
    for<'a> &'a C: IntoIterator<Item = &'a I> + Clone,
    I: HasId,
    F: Fn(&I) -> V,
    V: View
{
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            if let Some(item) = self.collection.into_iter().find(|item| item.id() == head) {
                (self.view_func)(item).fire(event, &id_path.tail());
            }
        }
    }

    fn render(&self, context: &Context) -> Node {
        Node::Group {
            children: self.collection
                .into_iter()
                .map(|item| {
                    let id = item.id();
                    (self.view_func)(item).render(&context.child(id.clone())).identify(id)
                })
                .collect(),
        }
    }
}
