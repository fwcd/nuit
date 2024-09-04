#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Angle, Bind, Color, Sector, ShapeExt, View, ZStack};

#[derive(Bind)]
struct PieChartView;

impl View for PieChartView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        ZStack::new((
            Sector::new(Angle::with_degrees(30.0), Angle::FULL, 0, 100)
                .fill(Color::BLUE),
            Sector::new(Angle::ZERO, Angle::with_degrees(30.0), 0, 150)
                .fill(Color::CYAN),
            Sector::new(Angle::with_degrees(10.0), Angle::with_degrees(20.0), 160, 180)
                .fill(Color::MAGENTA),
        ))
    }
}

fn main() {
    nuit::run_app(PieChartView);
}
