//! A declarative, cross-platform UI framework that uses native controls.
//! 
//! The API takes inspiration from contemporary reactive frameworks like
//! SwiftUI, Xilem and React. A central design goal is to avoid using too much
//! macro magic and instead heavily leverage associated types, traits and
//! generics.
//! 
//! A simple hello world program in Nuit takes only a single line:
//! 
//! ```no_run
//! use nuit::Text;
//! 
//! nuit::run_app(Text::new("Hello world!"));
//! ```

mod backend;
mod config;

pub use backend::*;
pub use config::*;

pub use nuit_core::*;
pub use nuit_derive::*;

impl Default for Backend {
    #[allow(unreachable_code)]
    fn default() -> Self {
        #[cfg(feature = "swiftui")]
        return Backend::SwiftUI;
        #[cfg(feature = "adwaita")]
        return Backend::Adwaita;
        panic!("A backend must be enabled via Nuit's crate features!");
    }
}

/// Blocks and presents the given view to the user.
pub fn run_app<T>(config: impl Into<Config<T>>) where T: View + 'static {
    let config: Config<T> = config.into();
    let backend = config.preferred_backend().unwrap_or_default();
    let view = config.into_view();
    let root = Root::new(view);

    match backend {
        #[cfg(feature = "swiftui")]
        Backend::SwiftUI => {
            let c_root = CRoot::from_typed(Box::new(root));

            #[cfg(target_vendor = "apple")]
            unsafe { nuit_bridge_swiftui::run_app(&c_root); }
            #[cfg(not(target_vendor = "apple"))]
            panic!("SwiftUI is not supported outside of Apple platforms!")
        }
        #[cfg(feature = "adwaita")]
        Backend::Adwaita => {
            nuit_bridge_adwaita::run_app(root);
        }
        #[allow(unreachable_patterns)]
        _ => panic!("The backend {backend:?} must be enabled via Nuit's crate features!"),
    }
}
