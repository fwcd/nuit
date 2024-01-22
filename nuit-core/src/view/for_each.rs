use std::marker::PhantomData;

use crate::{View, Node, Bind, Context, Identify, Identified};

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
    C: IntoIterator<Item = I> + Clone,
    I: Identify,
    F: Fn(I) -> V,
    V: Bind
{}

impl<C, I, F, V> View for ForEach<C, I, F, V>
where
    C: IntoIterator<Item = I> + Clone,
    I: Identify,
    F: Fn(I) -> V,
    V: View
{
    fn render(&mut self, context: &Context) -> Identified<Node> {
        context.identify(Node::Group {
            children: self.collection
                .clone()
                .into_iter()
                .map(|item| {
                    let id = item.id();
                    (self.view_func)(item).render(&context.child(id))
                })
                .collect(),
        })
    }
}
