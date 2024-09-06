#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Alignment, Animation, Bind, Button, Circle, ForEach, Frame, HStack, State, Text, VStack, Vec2, View, ViewExt};

const BALL_COUNT: usize = 4;
const ANIMATIONS: [Animation; BALL_COUNT] = [
    Animation::LINEAR,
    Animation::EASE_IN,
    Animation::EASE_OUT,
    Animation::EASE_IN_OUT,
];

#[derive(Bind, Default)]
struct AnimationsView {
    flips: State<[bool; BALL_COUNT]>,
}

impl View for AnimationsView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let flips = self.flips.clone();
        let width = 100.0;
        let radius = 20.0;
        VStack::new((
            VStack::new(
                ForEach::with_index_id(ANIMATIONS, |i, animation| {
                    let factor = if flips.get()[i] { 1.0 } else { -1.0 };
                    HStack::new((
                        Text::new(format!("{}", animation))
                            .frame_with(Alignment::Trailing, Frame::width(100)),
                        Circle::new()
                            .frame(radius)
                            .offset(Vec2::with_x(factor * (width - radius) / 2.0))
                            .frame(Frame::width(width)),
                    ))
                })
            ),
            Button::new(Text::new("Click me!"), move || {
                let mut value = flips.get();
                for (i, animation) in ANIMATIONS.into_iter().enumerate() {
                    value[i] = !value[i];
                    flips.set_with_animation(value, animation);
                }
            }),
        ))
    }
}

fn main() {
    nuit::run_app(AnimationsView::default());
}
