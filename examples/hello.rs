#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::prelude::*;
use nuit::{Text, VStack};

#[derive(Bind)]
struct HelloView;

impl View for HelloView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::from((
            Text::new("Hello"),
            Text::new("World"),
        ))
    }
}

fn main() {
    nuit::run_app(HelloView);
}
