use nui_shared::{View, CView};

pub fn bootstrap(view: impl View) {
    let ffi_view = CView::from(Box::new(view));

    unsafe {
        #[cfg(target_os = "macos")]
        {
            nui_swiftui_bridge::bootstrap(&ffi_view);
        }
    }
}
