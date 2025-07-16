#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Axis, Color, Font, ForEach, HStack, Rectangle, ScrollView, Style, Text, VStack};

#[derive(Bind)]
struct ScrollViewExample;

impl View for ScrollViewExample {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::from((
            Text::new("ScrollView Example").font(Font::TITLE).padding(20.0),
            
            // Vertical ScrollView (default)
            VStack::from((
                Text::new("Vertical ScrollView:").font(Font::HEADLINE),
                ScrollView::from(
                    VStack::from(
                        ForEach::new(0..50, |i| {
                            Text::new(format!("Item {}", i))
                                .frame((300, 40))
                                .padding(5.0)
                        })
                    )
                )
                .frame((350, 300))
                .border(Style::color(Color::GRAY), 1.0),
            )).padding(10.0),
            
            // Horizontal ScrollView  
            VStack::from((
                Text::new("Horizontal ScrollView:").font(Font::HEADLINE),
                ScrollView::horizontal(
                    HStack::from(
                        ForEach::new(0..20, |i| {
                            VStack::from((
                                Rectangle::new()
                                    .frame((100, 100))
                                    .foreground_style(Style::color(Color::new(
                                        (i as f64 * 0.05).min(1.0),
                                        0.5,
                                        1.0 - (i as f64 * 0.05).min(1.0),
                                        1.0
                                    ))),
                                Text::new(format!("Card {}", i)),
                            ))
                            .padding(10.0)
                        })
                    )
                )
                .frame((350, 150))
                .border(Style::color(Color::GRAY), 1.0),
            )).padding(10.0),
            
            // Both axes ScrollView
            VStack::from((
                Text::new("Both Axes ScrollView:").font(Font::HEADLINE),
                ScrollView::new(Axis::Both,
                    VStack::from(
                        ForEach::new(0..30, |row| {
                            HStack::from(
                                ForEach::new(0..10, |col| {
                                    Text::new(format!("{},{}", row, col))
                                        .frame((60, 40))
                                        .padding(2.0)
                                })
                            )
                        })
                    )
                )
                .frame((350, 200))
                .border(Style::color(Color::GRAY), 1.0),
            )).padding(10.0),
            
            // ScrollView without indicators
            VStack::from((
                Text::new("No Indicators:").font(Font::HEADLINE),
                ScrollView::vertical(
                    VStack::from(
                        ForEach::new(0..20, |i| {
                            Text::new(format!("Hidden scrollbar item {}", i))
                                .padding(10.0)
                        })
                    )
                )
                .show_indicators(false)
                .frame((350, 150))
                .border(Style::color(Color::GRAY), 1.0),
            )).padding(10.0),
        ))
    }
}

fn main() {
    nuit::run_app(ScrollViewExample);
}