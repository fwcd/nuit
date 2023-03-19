use std::ffi::{c_void, CString, c_char};

use crate::View;

/// An owned, FFI-style type-erased view.
/// Callers from another language that use functions returning
/// this type **will have to manually call the drop function**
/// once done, otherwise the memory will be leaked.
#[repr(C)]
pub struct CView {
    /// The opaque pointer to the owned underlying implementor of the `View` trait.
    wrapped_view: *const c_void,
    /// Renders the view to a JSON-serialized primitive tree.
    /// **Callers are responsible for freeing this string!**
    render_json: extern fn(*const CView) -> *const c_char,
    /// Frees (deallocates) this view. Must only be called once.
    /// **Note that Rust already calls this through the `Drop` trait**,
    /// therefore this method is only relevant to foreign callers.
    /// See the struct doc for more details.
    drop: extern fn(*const CView),
}

extern "C" fn render_json_impl<T>(c_view: *const CView) -> *const c_char where T: View {
    unsafe {
        let view = (*c_view).wrapped_view as *const T;
        let primitive = (*view).primitive();
        let json = serde_json::to_string(&primitive).expect("Could not serialize view");
        let c_string = CString::new(json).expect("Could not convert JSON to C string");
        c_string.into_raw()
    }
}

extern "C" fn drop_impl(c_view: *const CView) {
    unsafe {
        drop(Box::from_raw((*c_view).wrapped_view as *mut c_void));
    }
}

impl Drop for CView {
    fn drop(&mut self) {
        drop_impl(self as *const CView)
    }
}

impl<T> From<Box<T>> for CView where T: View {
    fn from(value: Box<T>) -> Self {
        Self {
            wrapped_view: Box::into_raw(value) as *const c_void,
            render_json: render_json_impl::<T>,
            drop: drop_impl,
        }
    }
}
