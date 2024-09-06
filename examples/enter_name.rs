#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Text, VStack, View, ViewExt, State, HStack, TextField, Bind, Insets, Frame, ForEach, clone};

#[derive(Bind, Default)]
struct EnterNameView {
    raw_names: State<String>,
}

impl View for EnterNameView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let raw_names = self.raw_names.clone();
        VStack::new((
            HStack::new((
                Text::new("Please enter some names:"),
                TextField::new(raw_names.binding()),
            )),
            ForEach::new(raw_names.get().split(",").map(|s| s.trim().to_owned()).filter(|s| !s.is_empty()), |name| {
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
    nuit::run_app(EnterNameView::default());
}
