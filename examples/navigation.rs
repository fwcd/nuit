#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, NavigationLink, NavigationStack, Text, VStack};

#[derive(Bind, Default)]
struct NavigationView {
    i: i32,
}

impl NavigationView {
    pub const fn new(i: i32) -> Self {
        Self { i }
    }
}

impl View for NavigationView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let i = self.i;
        NavigationStack::from(
            VStack::from((
                Text::new(format!("This is page {i}")),
                NavigationLink::with_text("Next", i + 1),
            ))
            .navigation_destination(Self::new)
        )
    }
}

fn main() {
    nuit::run_app(NavigationView::new(0));
}
