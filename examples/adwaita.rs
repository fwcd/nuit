#![feature(type_alias_impl_trait, impl_trait_in_assoc_type)]

use nuit::{Backend, ConfigBuilder, Text};

fn main() {
    nuit::run_app(
        ConfigBuilder::from(Text::new("Hello Adwaita!"))
            .preferred_backend(Backend::Adwaita)
    );
}
