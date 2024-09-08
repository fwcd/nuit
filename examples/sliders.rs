#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Circle, Font, FontDesign, FontSize, Frame, HStack, Slider, Text, VStack, Vec2};

#[derive(Bind, Default)]
struct SlidersView {
    position: State<Vec2<f64>>,
}

impl View for SlidersView {
    type Body = impl View;

    #[allow(clippy::cast_possible_truncation)]
    fn body(&self) -> Self::Body {
        let position = self.position.clone();
        let width = 400.0;
        let height = 300.0;
        let slider_width = 100.0;

        VStack::from((
            Circle::new()
                .frame(10)
                .offset(position.get())
                .frame((width, height)),

            HStack::from((
                Text::new(format!("X: {:>4}", position.get().x as i32)),
                Slider::with_default_step(
                    position.project(|p| &mut p.x),
                    -(width / 2.0)..=(width / 2.0)
                )
                .frame(Frame::with_width(slider_width)),

                Text::new(format!("Y: {:>4}", position.get().y as i32)),
                Slider::with_default_step(
                    position.project(|p| &mut p.y),
                    -(height / 2.0)..=(height / 2.0)
                )
                .frame(Frame::with_width(slider_width)),
            )
            .font(Font::system(FontSize::BODY, FontDesign::Monospaced, None))),
        ))
    }
}

fn main() {
    nuit::run_app(SlidersView::default());
}
