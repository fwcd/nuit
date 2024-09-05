#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Bind, Circle, Color, ShapeExt, State, Text, View, ViewExt};

#[derive(Bind)]
struct GesturesView {
    color: State<Color>,
}

impl GesturesView {
    pub fn new() -> Self {
        Self {
            color: State::new(Color::BLACK),
        }
    }
}

impl View for GesturesView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let color = self.color.clone();
        Circle::new()
            .fill(color.get())
            .overlay(Text::new("Tap me!"))
            .on_tap(move || {
                color.set(Color::random_rgb());
            })
    }
}

fn main() {
    nuit::run_app(GesturesView::new());
}
