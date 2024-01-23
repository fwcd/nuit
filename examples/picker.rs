#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{VStack, View, Bind, State};
use nuit_core::{Picker, Id, Text};

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
        VStack::new((
            Picker::new("Favorite Color", self.selection.binding(), (
                Text::new("Red"),
                Text::new("Green"),
                Text::new("Yellow"),
                Text::new("Blue"),
            )),
        ))
    }
}

fn main() {
    nuit::run_app(PickerView::new());
}
