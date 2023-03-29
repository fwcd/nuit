pub use nui_shared::*;
pub use nui_derive::*;

/// Blocks and presents the given view to the user.
pub fn run_app(view: impl View) {
    let root = Root::new(view);
    let c_root = CRoot::from(Box::new(root));

    unsafe {
        #[cfg(target_os = "macos")]
        {
            nui_swiftui_bridge::run_app(&c_root);
        }
    }
}
