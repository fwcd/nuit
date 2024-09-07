#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Text, VStack, View, Bind, Button, State};

#[derive(Bind)]
struct IncrementView<F> {
    action: F,
}

impl<F> IncrementView<F> {
    pub const fn new(action: F) -> Self {
        Self { action }
    }
}

impl<F> View for IncrementView<F> where F: Fn() + Clone + 'static {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let action = self.action.clone();
        Button::with_text("Increment", move || {
            action();
        })
    }
}

#[derive(Bind, Default)]
struct CounterView {
    count: State<i32>,
}

impl View for CounterView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let count = self.count.clone();
        VStack::from((
            Text::new(format!("Count: {}", count.get())),
            IncrementView::new(move || {
                count.set(count.get() + 1);
            })
        ))
    }
}

fn main() {
    nuit::run_app(CounterView::default());
}
