use std::ffi::{c_void, CString, c_char};

use crate::View;

/// An owned, FFI-style type-erased view.
#[repr(C)]
pub struct CView {
    /// The opaque pointer to the owned underlying implementor of the `View` trait.
    wrapped_view: *const c_void,
    /// Renders the view to an owned JSON-serialized primitive tree.
    /// **Callers are responsible for calling `nui_drop_string` on this string!**
    render_json: extern fn(*const CView) -> *const c_char,
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

#[no_mangle]
pub extern "C" fn nui_c_string_drop(raw_string: *const c_char) {
    unsafe {
        drop(CString::from_raw(raw_string as *mut c_char))
    }
}

impl Drop for CView {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.wrapped_view as *mut c_void));
        }
    }
}

impl<T> From<Box<T>> for CView where T: View {
    fn from(value: Box<T>) -> Self {
        Self {
            wrapped_view: Box::into_raw(value) as *const c_void,
            render_json: render_json_impl::<T>,
        }
    }
}
