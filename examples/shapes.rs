#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Bind, Capsule, Circle, Color, Ellipse, Frame, Rectangle, RoundedRectangle, Style, VStack, View, ViewExt};

#[derive(Bind)]
struct ShapesView;

impl View for ShapesView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::new(
            (
                Capsule::new(),
                Circle::new(),
                Ellipse::new(),
                Rectangle::new(),
                RoundedRectangle::with_corner_radius(15.0).fill(Style::color(Color::RED)),
            )
            .frame(Frame::exact(100, 50))
        )
    }
}

fn main() {
    nuit::run_app(ShapesView);
}
