#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Alignment, Animation, Bind, Button, Circle, ForEach, Frame, HStack, Rectangle, State, Text, VStack, Vec2, View, ViewExt, ZStack};

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
        let width = 200.0;
        let radius = 10.0;
        let inner_width = width - 2.0 * radius;
        VStack::new((
            VStack::new(
                ForEach::with_index_id(ANIMATIONS, |i, animation| {
                    let factor = if flips.get()[i] { 1.0 } else { -1.0 };
                    HStack::new((
                        Text::new(format!("{}", animation))
                            .frame_with(Alignment::Trailing, Frame::with_width(100)),
                        ZStack::new((
                            Rectangle::new()
                                .frame(Frame::exact(inner_width, 2)),
                            Circle::new()
                                .frame(2.0 * radius)
                                .offset(Vec2::with_x(factor * inner_width / 2.0))
                        ))
                        .frame(Frame::with_width(width)),
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
