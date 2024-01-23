#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{VStack, View, Bind, State, Picker, Id, Text, Button, clone};

#[derive(Bind)]
struct PickerView {
    selection: State<Id>,
}

impl PickerView {
    fn new() -> Self {
        Self { selection: State::new(0) }
    }
}

impl View for PickerView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let selection = self.selection.clone();
        VStack::new((
            Picker::new("Favorite Color", selection.binding(), (
                Text::new("Red"),
                Text::new("Green"),
                Text::new("Yellow"),
                Text::new("Blue"),
            )),
            Button::new(Text::new("Choose next"), clone!(selection => move || {
                if let Id::Index(i) = selection.get() {
                    selection.set((i + 1) % 4);
                }
            }))
        ))
    }
}

fn main() {
    nuit::run_app(PickerView::new());
}
