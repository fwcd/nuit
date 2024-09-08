#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Circle, HStack, Slider, Vec2, VStack};

#[derive(Bind, Default)]
struct SlidersView {
    position: State<Vec2<f64>>,
}

impl View for SlidersView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let position = self.position.clone();
        let width = 400.0;
        let height = 300.0;
        VStack::from((
            Circle::new()
                .offset(position.get())
                .frame((width, height)),
            HStack::from((
                Slider::with_default_step(
                    position.project(|p| &mut p.x),
                    -(width / 2.0)..=(width / 2.0)
                ),
                Slider::with_default_step(
                    position.project(|p| &mut p.y),
                    -(height / 2.0)..=(height / 2.0)
                ),
            )),
        ))
    }
}

fn main() {
    nuit::run_app(SlidersView::default());
}
