#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::prelude::*;
use nuit::{Text, List};

#[derive(Bind)]
struct ListView;

impl View for ListView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        List::new((
            Text::new("Hello, world!"),
            Text::new("This is an item."),
            Text::new("This is another."),
            Text::new("You get the idea."),
        ))
    }
}

fn main() {
    nuit::run_app(ListView);
}
