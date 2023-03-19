use nui_shared::{View, CView};

/// Blocks and presents the given view to the user.
pub fn run_app(view: impl View) {
    let ffi_view = CView::from(Box::new(view));

    unsafe {
        #[cfg(target_os = "macos")]
        {
            nui_swiftui_bridge::run_app(&ffi_view);
        }
    }
}
