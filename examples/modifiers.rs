#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Button, Circle, Color, Font, HStack, Rectangle, RoundedRectangle, Style, Text, VStack, Vec2};

#[derive(Bind)]
struct ModifiersView;

impl View for ModifiersView {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::from((
            Text::new("New Modifiers Demo").font(Font::TITLE).padding(20.0),
            
            // Visual Effects
            Text::new("Visual Effects").font(Font::TITLE2).padding(10.0),
            HStack::from((
                VStack::from((
                    Text::new("Border"),
                    Rectangle::new()
                        .frame((80, 80))
                        .foreground_style(Style::color(Color::BLUE))
                        .border(Style::color(Color::RED), 3.0),
                )),
                
                VStack::from((
                    Text::new("Shadow"),
                    Circle::new()
                        .frame(80)
                        .foreground_style(Style::color(Color::GREEN))
                        .shadow(Style::color(Color::new(0.0, 0.0, 0.0, 0.5)), 10.0, Vec2::new(5.0, 5.0)),
                )),
                
                VStack::from((
                    Text::new("Blur"),
                    Rectangle::new()
                        .frame((80, 80))
                        .foreground_style(Style::color(Color::new(1.0, 0.5, 0.0, 1.0)))
                        .blur(2.0),
                )),
                
                VStack::from((
                    Text::new("Corner Radius"),
                    Rectangle::new()
                        .frame((80, 80))
                        .foreground_style(Style::color(Color::new(0.5, 0.0, 0.5, 1.0)))
                        .corner_radius(15.0),
                )),
            )),
            
            // Color Adjustments
            Text::new("Color Adjustments").font(Font::TITLE2).padding(10.0),
            HStack::from((
                VStack::from((
                    Text::new("Grayscale"),
                    RoundedRectangle::with_corner_radius(10.0)
                        .frame((80, 80))
                        .foreground_style(Style::color(Color::RED))
                        .grayscale(1.0),
                )),
                
                VStack::from((
                    Text::new("Brightness"),
                    RoundedRectangle::with_corner_radius(10.0)
                        .frame((80, 80))
                        .foreground_style(Style::color(Color::BLUE))
                        .brightness(0.3),
                )),
                
                VStack::from((
                    Text::new("Contrast"),
                    RoundedRectangle::with_corner_radius(10.0)
                        .frame((80, 80))
                        .foreground_style(Style::color(Color::GREEN))
                        .contrast(2.0),
                )),
                
                VStack::from((
                    Text::new("Saturation"),
                    RoundedRectangle::with_corner_radius(10.0)
                        .frame((80, 80))
                        .foreground_style(Style::color(Color::new(1.0, 0.5, 0.0, 1.0)))
                        .saturation(3.0),
                )),
            )),
            
            // State modifiers
            Text::new("State & Layout").font(Font::TITLE2).padding(10.0),
            HStack::from((
                VStack::from((
                    Text::new("Disabled"),
                    HStack::from((
                        Button::with_text("Normal", || println!("Normal button clicked"))
                            .padding(10.0),
                        Button::with_text("Disabled", || println!("This won't print"))
                            .padding(10.0)
                            .disabled(true),
                    )),
                )),
                
                VStack::from((
                    Text::new("Z-Index"),
                    HStack::from((
                        Rectangle::new()
                            .frame((60, 60))
                            .foreground_style(Style::color(Color::RED))
                            .z_index(1.0),
                        Rectangle::new()
                            .frame((60, 60))
                            .foreground_style(Style::color(Color::BLUE))
                            .offset(Vec2::new(-30.0, 0.0))
                            .z_index(0.0),
                    )),
                )),
                
                VStack::from((
                    Text::new("Clipped"),
                    Circle::new()
                        .frame(80)
                        .foreground_style(Style::color(Color::YELLOW))
                        .overlay(
                            Rectangle::new()
                                .frame((100, 20))
                                .foreground_style(Style::color(Color::RED))
                        )
                        .clipped(),
                )),
            )),
        ))
        .padding(20.0)
    }
}

fn main() {
    nuit::run_app(ModifiersView);
}