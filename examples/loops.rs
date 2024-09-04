#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Bind, ForEach, HStack, Text, VStack, View};

#[derive(Bind)]
struct LoopsView;

impl View for LoopsView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::new((
            HStack::new(
                ForEach::new(vec!["Hello", "World"], |&s| {
                    Text::new(s)
                }),
            ),
            HStack::new(
                ForEach::new(vec![42, 1, 2, 3], |&v| {
                    Text::new(format!("{v}"))
                }),
            ),
        ))
    }
}

fn main() {
    nuit::run_app(LoopsView);
}
