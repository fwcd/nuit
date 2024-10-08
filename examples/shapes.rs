#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Capsule, Circle, Color, Ellipse, Rectangle, RoundedRectangle, VStack};

#[derive(Bind)]
struct ShapesView;

impl View for ShapesView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::from(
            (
                Capsule::new(),
                Circle::new(),
                Ellipse::new(),
                Rectangle::new(),
                RoundedRectangle::with_corner_radius(15.0)
                    .fill(Color::RED),
            )
            .frame((100, 50))
        )
    }
}

fn main() {
    nuit::run_app(ShapesView);
}
