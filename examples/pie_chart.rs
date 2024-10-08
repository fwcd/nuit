#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Alignment, Angle, Color, Insets, Sector, Text, ZStack};

#[derive(Bind)]
struct PieChartView;

impl View for PieChartView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        ZStack::from((
            Sector::new(Angle::with_degrees(30.0), Angle::FULL, 0)
                .fill(Color::BLUE)
                .frame(160)
                .overlay_at(Alignment::LEADING, Text::new("Majority").padding(Insets::default())),
            Sector::new(Angle::ZERO, Angle::with_degrees(30.0), 0)
                .fill(Color::CYAN)
                .frame(240),
            Sector::new(Angle::with_degrees(10.0), Angle::with_degrees(20.0), 0.9)
                .fill(Color::MAGENTA)
                .frame(280),
        ))
    }
}

fn main() {
    nuit::run_app(PieChartView);
}
