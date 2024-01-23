use std::marker::PhantomData;

use crate::{View, Node, Bind, Context, Identify, Identified, Event, IdPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForEach<C, I, F, V> {
    collection: C,
    view_func: F,
    phantom_item: PhantomData<I>,
    phantom_view: PhantomData<V>,
}

impl<C, I, F, V> ForEach<C, I, F, V> {
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
    I: Identify,
    F: Fn(&I) -> V,
    V: Bind
{}

impl<C, I, F, V> View for ForEach<C, I, F, V>
where
    for<'a> &'a C: IntoIterator<Item = &'a I> + Clone,
    I: Identify,
    F: Fn(&I) -> V,
    V: View
{
    fn fire(&self, event: &Event, id_path: &IdPath) {
        if let Some(head) = id_path.head() {
            if let Some(item) = self.collection.into_iter().find(|item| item.id() == head) {
                (self.view_func)(item).fire(event, &id_path.child(item.id()));
            }
        }
    }

    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Group {
            children: self.collection
                .into_iter()
                .map(|item| {
                    let id = item.id();
                    (self.view_func)(item).render(&context.child(id))
                })
                .collect(),
        })
    }
}
