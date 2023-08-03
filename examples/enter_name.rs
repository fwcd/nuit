#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Text, VStack, View, State, HStack, TextField, Bind};

#[derive(Bind)]
struct EnterNameView {
    name: State<String>,
}

impl EnterNameView {
    fn new() -> Self {
        Self { name: State::new("") }
    }
}

impl View for EnterNameView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let name = self.name.clone();
        VStack::new((
            HStack::new((
                Text::new("Please enter your name:"),
                TextField::new(name.binding()),
            )),
            Text::new(format!("Hi {}!", name.get())),
        ))
    }
}

fn main() {
    nuit::run_app(EnterNameView::new());
}
