#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, NavigationLink, NavigationSplitView, NavigationStack, Text, VStack};

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

#[derive(Bind)]
struct NavigationView;

impl View for NavigationView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        NavigationSplitView::with_sidebar({
            
        }).with_detail(
            NavigationStack::from(
                NavigationContent::new(0)
            )
        )
    }
}

fn main() {
    nuit::run_app(NavigationView);
}
