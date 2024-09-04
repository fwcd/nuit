#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Angle, Bind, Color, Sector, ShapeExt, View, ViewExt, ZStack};

#[derive(Bind)]
struct PieChartView;

impl View for PieChartView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        ZStack::new((
            Sector::new(Angle::with_degrees(30.0), Angle::FULL, 0)
                .fill(Color::BLUE)
                .frame(100),
            Sector::new(Angle::ZERO, Angle::with_degrees(30.0), 0)
                .fill(Color::CYAN)
                .frame(150),
            Sector::new(Angle::with_degrees(10.0), Angle::with_degrees(20.0), 0.9)
                .fill(Color::MAGENTA)
                .frame(180),
        ))
    }
}

fn main() {
    nuit::run_app(PieChartView);
}
