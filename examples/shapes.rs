#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Bind, Capsule, Circle, Ellipse, Rectangle, RoundedRectangle, VStack, View};

#[derive(Bind)]
struct ShapesView;

impl View for ShapesView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::new((
            Capsule::new(),
            Circle::new(),
            Ellipse::new(),
            Rectangle::new(),
            RoundedRectangle::with_corner_radius(15.0),
        ))
    }
}

fn main() {
    nuit::run_app(ShapesView);
}
