#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Text, VStack, View, Bind};

struct HelloView;

impl Bind for HelloView {}

impl View for HelloView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::new((
            Text::new("Hello"),
            Text::new("World"),
        ))
    }
}

fn main() {
    nuit::run_app(HelloView);
}
