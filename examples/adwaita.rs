#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Backend, Bind, Button, ConfigBuilder, State, Text, VStack, View};

#[derive(Bind)]
struct CounterView {
    count: State<i32>,
}

impl CounterView {
    fn new() -> Self {
        Self { count: State::new(0) }
    }
}

impl View for CounterView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let count = self.count.clone();
        VStack::new((
            Text::new(format!("Count: {}", count.get())),
            Button::new(Text::new("Increment"), move || {
                count.set(count.get() + 1);
            })
        ))
    }
}

fn main() {
    nuit::run_app(
        ConfigBuilder::from(CounterView::new())
            .preferred_backend(Backend::Adwaita)
    );
}