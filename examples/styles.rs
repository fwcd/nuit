#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Bind, BlendMode, Circle, Color, ForEach, HStack, Shadow, ShadowKind, Style, Text, VStack, Vec2, View, ViewExt, Zero};

#[derive(Bind)]
struct StylesView<const N: usize> {
    styles: [(&'static str, Vec<Style>); N],
}

impl<const N: usize> StylesView<N> {
    pub fn new(styles: [(&'static str, Vec<Style>); N]) -> Self {
        Self { styles }
    }
}

impl<const N: usize> View for StylesView<N> {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::new(
            ForEach::with_index_id(&self.styles, |_, (label, styles)| {
                HStack::new((
                    Text::new(*label),
                    ForEach::with_index_id(styles, |_, style| {
                        Circle::new()
                            .frame(20)
                            .foreground_style(style.clone())
                    }),
                ))
            })
        )
    }
}

fn main() {
    nuit::run_app(StylesView::new([
        ("Color", [
            Color::RED,
            Color::YELLOW,
            Color::GREEN,
            Color::BLUE,
            Color::MAGENTA
        ].map(Style::color).into()),
        ("Hierarchical", (0..5).map(Style::hierarchical).collect()),
        ("Semantic", [
            Style::FOREGROUND,
            Style::BACKGROUND,
            Style::SELECTION,
            Style::SEPARATOR, 
            Style::TINT,
            Style::PLACEHOLDER,
            Style::LINK,
            Style::FILL,
            Style::WINDOW_BACKGROUND
        ].into()),
        ("Blend mode", [
            BlendMode::Normal,
            BlendMode::Darken,
            BlendMode::Multiply,
            BlendMode::ColorBurn,
            BlendMode::PlusDarker,
            BlendMode::Lighten,
            BlendMode::Screen
        ].map(|bm| Style::FOREGROUND.blend_mode(bm)).into()),
        ("Opacity", (0..5).map(|i| Style::FOREGROUND.opacity(i as f64 * 2.0 / 10.0)).collect()),
        ("Shadow", [
            ShadowKind::Drop,
            ShadowKind::Inner
        ].into_iter().flat_map(|sk| (1..4).map(move |r|
            Shadow::new(sk, Color::MAGENTA, r as f64 * 2.0, Vec2::ZERO)
        )).map(|s| Style::FOREGROUND.shadow(s)).collect()),
    ]));
}

