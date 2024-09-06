#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{clone, Angle, Animation, Bind, Button, Color, HStack, Rectangle, State, Text, VStack, View, ViewExt};

#[derive(Bind)]
struct ScaleRotateView {
    scale: State<f64>,
    rotation: State<Angle>,
}

impl ScaleRotateView {
    pub fn new() -> Self {
        Self {
            scale: State::new(1.0),
            rotation: State::new(Angle::ZERO),
        }
    }
}

impl View for ScaleRotateView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let scale = self.scale.clone();
        let rotation = self.rotation.clone();
        VStack::with_spacing(100, (
            Rectangle::new()
                .overlay(Text::new("Hello world!").foreground_style(Color::GRAY))
                .rotation_effect(rotation.get())
                .scale_effect(scale.get())
                .frame(100),
            HStack::new((
                Button::new(Text::new("Rotate"), move || {
                    rotation.set_with(Animation::LINEAR, rotation.get() + Angle::QUARTER);
                }),
                Button::new(Text::new("Scale up"), clone!(scale => move || {
                    scale.set_with(Animation::LINEAR, scale.get() + 0.5);
                })),
                Button::new(Text::new("Scale down"), move || {
                    scale.set_with(Animation::LINEAR, scale.get() - 0.5);
                }),
            ))
        ))
    }
}

fn main() {
    nuit::run_app(ScaleRotateView::new());
}
