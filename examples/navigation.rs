#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, NavigationStack, Text};

#[derive(Bind)]
struct NavigationView;

impl View for NavigationView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        NavigationStack::from(
            Text::new("Hello!"),
        )
    }
}

fn main() {
    nuit::run_app(NavigationView);
}
