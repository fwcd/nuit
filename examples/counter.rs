#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Access, Bind, Button, State, Text, VStack, View};

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
            Button::with_text("Increment", move || {
                count.set(count.get() + 1);
            })
        ))
    }
}

fn main() {
    nuit::run_app(CounterView::default());
}
