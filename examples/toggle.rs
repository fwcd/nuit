#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{prelude::*, Toggle, VStack, HStack, Text, Font, FontLevel, Insets};

#[derive(Bind, Default)]
struct ToggleExample {
    is_on: State<bool>,
    airplane_mode: State<bool>,
    wifi_enabled: State<bool>,
}

impl ToggleExample {
    fn new() -> Self {
        Self {
            is_on: State::new(false),
            airplane_mode: State::new(false),
            wifi_enabled: State::new(true),
        }
    }
}

impl View for ToggleExample {
    type Body = impl View;

    fn body(&self) -> Self::Body {
        VStack::with_spacing(20.0, (
            Text::new("Toggle Examples").font(Font::with_level(FontLevel::LargeTitle)),
            
            VStack::with_spacing(10.0, (
                HStack::from((
                    Text::new("Basic Toggle"),
                    Toggle::new(self.is_on.binding()),
                )),
                
                HStack::from((
                    Text::new("Airplane Mode"),
                    Toggle::new(self.airplane_mode.binding()),
                )),
                
                HStack::from((
                    Text::new("Wi-Fi"),
                    Toggle::new(self.wifi_enabled.binding()),
                )),
            )),
            
            VStack::with_spacing(5.0, (
                Text::new("Current States:").font(Font::with_level(FontLevel::Headline)),
                Text::new(format!("Basic Toggle: {}", if self.is_on.get() { "ON" } else { "OFF" })),
                Text::new(format!("Airplane Mode: {}", if self.airplane_mode.get() { "ON" } else { "OFF" })),
                Text::new(format!("Wi-Fi: {}", if self.wifi_enabled.get() { "Enabled" } else { "Disabled" })),
            )),
        ))
        .padding(Insets::default())
    }
}

fn main() {
    nuit::run_app(ToggleExample::new());
}