#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Text, VStack, View, ViewExt, State, HStack, TextField, Bind, Insets, Frame, ForEach, clone};

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
                Text::new("Please enter some names:"),
                TextField::new(name.binding()),
            )),
            ForEach::new(name.get().split(" ").map(|s| s.trim().to_owned()), |name| {
                Text::new(format!("Hi {}!", name))
                    .on_appear(clone!(name => move || println!("A wild {} appeared!", name)))
                    .on_disappear(clone!(name => move || println!("{} disappeared!", name)))
            }),
        ))
        .padding(Insets::default())
        .frame(Frame::width(400))
    }
}

fn main() {
    nuit::run_app(EnterNameView::new());
}
