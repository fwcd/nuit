#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Bind, ForEach, HStack, Text, VStack, View};

#[derive(Bind)]
struct LoopsView;

impl View for LoopsView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::new((
            HStack::new(
                ForEach::new(["Hello", "World"], |s| {
                    Text::new(s)
                }),
            ),
            HStack::new(
                ForEach::new(0..10, |i| {
                    Text::new(format!("{i}"))
                })
            ),
        ))
    }
}

fn main() {
    nuit::run_app(LoopsView);
}
