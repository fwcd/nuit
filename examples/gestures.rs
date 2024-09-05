#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Bind, Circle, Color, HStack, ShapeExt, State, Text, View, ViewExt};

#[derive(Bind)]
struct TapView {
    taps: usize,
    color: State<Color>,
}

impl TapView {
    pub fn new(taps: usize) -> Self {
        Self {
            taps,
            color: State::new(Color::BLACK),
        }
    }
}

impl View for TapView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let color = self.color.clone();
        Circle::new()
            .fill(color.get())
            .frame(100)
            .overlay(Text::new("Tap me!"))
            .on_taps(self.taps, move || {
                color.set(Color::random_rgb());
            })
    }
}

#[derive(Bind)]
struct GesturesView;

impl View for GesturesView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        HStack::new((
            TapView::new(1),
            TapView::new(2),
        ))
    }
}

fn main() {
    nuit::run_app(GesturesView);
}