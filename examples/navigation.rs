#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, NavigationLink, NavigationStack, Text, VStack};

#[derive(Bind, Default)]
struct NavigationContent {
    i: i32,
}

impl NavigationContent {
    pub const fn new(i: i32) -> Self {
        Self { i }
    }
}

impl View for NavigationContent {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let i = self.i;
        VStack::from((
            Text::new(format!("This is page {i}")),
            NavigationLink::with_text("Next", i + 1),
        ))
        .navigation_destination(Self::new)
    }
}

fn main() {
    nuit::run_app(NavigationStack::from(
        NavigationContent::new(0)
    ));
}
