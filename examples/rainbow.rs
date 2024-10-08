#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Angle, Color, ForEach, Insets, Sector, ZStack};

#[derive(Bind)]
struct RainbowView;

impl View for RainbowView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let total = 7i32;
        let fraction = |i| f64::from(i) / f64::from(total);
        let radius = |i| 50.0 + fraction(i) * 100.0;
        ZStack::from(
            ForEach::new((0..total).rev(), |i| {
                let outer_radius = radius(i);
                let inner_radius = radius(-1);
                Sector::new(Angle::HALF, Angle::FULL, inner_radius / outer_radius)
                    .fill(Color::with_hsv(Angle::with_fractional(fraction(total - 1 - i)), 1.0, 1.0))
                    .frame(outer_radius * 2.0)
            })
        )
        .padding(Insets::default())
    }
}

fn main() {
    nuit::run_app(RainbowView);
}
