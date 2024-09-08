#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Font, FontDesign, FontLevel, FontWeight, ForEach, HStack, Text, VStack};

#[derive(Bind)]
struct FontsView;

impl View for FontsView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        HStack::with_spacing(50, (
            VStack::from(
                ForEach::with_index_id([
                    FontLevel::ExtraLargeTitle2,
                    FontLevel::ExtraLargeTitle,
                    FontLevel::LargeTitle,
                    FontLevel::Title,
                    FontLevel::Title2,
                    FontLevel::Title3,
                    FontLevel::Headline,
                    FontLevel::Subheadline,
                    FontLevel::Body,
                    FontLevel::Callout,
                    FontLevel::Caption,
                    FontLevel::Caption2,
                    FontLevel::Footnote,
                ], |_, level| {
                    Text::new(format!("{level:?}"))
                        .font(Font::with_level(level))
                })
            ),
            VStack::from(
                ForEach::with_index_id([
                    FontWeight::Black,
                    FontWeight::Bold,
                    FontWeight::Heavy,
                    FontWeight::Light,
                    FontWeight::Medium,
                    FontWeight::Regular,
                    FontWeight::Semibold,
                    FontWeight::Thin,
                    FontWeight::UltraLight,
                ], |_, weight| {
                    Text::new(format!("{weight:?}"))
                        .font(Font::system(18, None, Some(weight)))
                })
            ),
            VStack::from(
                ForEach::with_index_id([
                    FontDesign::Default,
                    FontDesign::Monospaced,
                    FontDesign::Rounded,
                    FontDesign::Serif,
                ], |_, design| {
                    Text::new(format!("{design:?}"))
                        .font(Font::system(18, Some(design), None))
                })
            ),
            VStack::from(
                ForEach::new((0..10).rev(), |i| {
                    let size = i * 4 + 8;
                    Text::new(format!("{size}"))
                        .font(Font::with_size(size))
                })
            ),
        ))
    }
}

fn main() {
    nuit::run_app(FontsView);
}
