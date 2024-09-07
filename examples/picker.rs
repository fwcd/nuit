#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::prelude::*;
use nuit::{clone, Button, Id, Picker, Text, VStack};

#[derive(Bind, Default)]
struct PickerView {
    selection: State<Id>,
}

impl View for PickerView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let selection = self.selection.clone();
        VStack::from((
            Picker::new("Favorite Color", selection.binding(), (
                Text::new("Red"),
                Text::new("Green"),
                Text::new("Yellow"),
                Text::new("Blue"),
            )),
            Button::with_text("Choose next", clone!(selection => move || {
                if let Id::Index(i) = selection.get() {
                    selection.set((i + 1) % 4);
                }
            }))
        ))
    }
}

fn main() {
    nuit::run_app(PickerView::default());
}
