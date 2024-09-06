#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Alignment, Bind, ForEach, Text, View, ViewExt, ZStack};

#[derive(Bind, Default)]
struct AlignmentsView;

impl View for AlignmentsView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        ZStack::new(
            ForEach::with_index_id([
                ("Top leading", Alignment::TOP_LEADING),
                ("Top", Alignment::TOP),
                ("Top trailing", Alignment::TOP_TRAILING),
                ("Leading", Alignment::LEADING),
                ("Center", Alignment::CENTER),
                ("Trailing", Alignment::TRAILING),
                ("Bottom leading", Alignment::BOTTOM_LEADING),
                ("Bottom", Alignment::BOTTOM),
                ("Bottom trailing", Alignment::BOTTOM_TRAILING),
            ], |_, (label, alignment)| {
                Text::new(label)
                    .frame_with(alignment, (400, 300))
            })
        )
    }
}

fn main() {
    nuit::run_app(AlignmentsView);
}
