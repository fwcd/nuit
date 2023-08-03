mod backend;
mod config;

pub use backend::*;
pub use config::*;

pub use nuit_core::*;
pub use nuit_derive::*;

impl Default for Backend {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        return Backend::SwiftUI;
        #[cfg(not(target_os = "macos"))]
        return Backend::Relm;
    }
}

/// Blocks and presents the given view to the user.
pub fn run_app<T>(config: impl Into<Config<T>>) where T: View {
    let config: Config<T> = config.into();
    let backend = config.preferred_backend().unwrap_or_default();
    let view = config.into_view();
    let root = Root::new(view);

    match backend {
        Backend::SwiftUI => {
            let c_root = CRoot::from(Box::new(root));
            #[cfg(target_os = "macos")]
            unsafe { nuit_bridge_swiftui::run_app(&c_root); }
            #[cfg(not(target_os = "macos"))]
            panic!("SwiftUI is not supported outside of macOS!")
        }
        Backend::Relm => {
            panic!("Relm is not supported (yet)")
        }
    }
}
