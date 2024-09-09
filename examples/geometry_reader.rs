#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, GeometryReader, Insets, Text};

#[derive(Bind)]
struct GeometryReaderView;

impl View for GeometryReaderView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        GeometryReader::new(|geometry| {
            Text::new(format!("This view has width {} and height {}", geometry.width(), geometry.height()))
                .padding(Insets::default())
        })
    }
}

fn main() {
    nuit::run_app(GeometryReaderView);
}
