#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Animation, Bind, Circle, Color, DragEventKind, HStack, ShapeExt, State, Text, Vec2, View, ViewExt};

#[derive(Bind)]
struct TapView {
    taps: usize,
    color: State<Color>,
}

impl TapView {
    pub fn new(taps: usize) -> Self {
        Self {
            taps,
            color: State::new(Color::BLACK),
        }
    }
}

impl View for TapView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let color = self.color.clone();
        Circle::new()
            .fill(color.get())
            .frame(150)
            .overlay(
                Text::new(match self.taps {
                    1 => "Tap me!".to_owned(),
                    2 => "Double-tap me!".to_owned(),
                    i => format!("Tap me {i} times!"),
                })
                .foreground_style(color.get().invert_rgb())
            )
            .on_taps(self.taps, move || {
                color.set(Color::random_rgb());
            })
    }
}

#[derive(Bind, Default)]
struct DragView {
    offset: State<Vec2<f64>>,
}

impl View for DragView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        let offset = self.offset.clone();
        Circle::new()
            .fill(Color::BLACK)
            .frame(150)
            .overlay(
                Text::new("Drag me!")
                    .foreground_style(Color::WHITE)
            )
            .offset(offset.get())
            .on_drag(move |event| {
                match event.kind() {
                    DragEventKind::Updated => offset.set(event.translation()),
                    DragEventKind::Ended => offset.set_with(Animation::default(), Vec2::default()),
                }
            })
    }
}

#[derive(Bind)]
struct GesturesView;

impl View for GesturesView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        HStack::from((
            TapView::new(1),
            TapView::new(2),
            DragView::default(),
        ))
    }
}

fn main() {
    nuit::run_app(GesturesView);
}
