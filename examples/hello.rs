#![feature(type_alias_impl_trait)]

use nui::{Text, VStack, View};

struct HelloView;

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
    nui::run_app(HelloView);
}
