use std::{ffi::{c_void, c_char, CString, CStr}, str};

use crate::{Root, View};

/// A C/FFI-compatible wrapper around `Root<T>`.
#[repr(C)]
pub struct CRoot {
    /// The opaque pointer to the owned underlying Rust `NuitRoot<T>`.
    wrapped: *mut c_void,
    /// Renders the view to an owned JSON-serialized primitive tree.
    /// **Callers are responsible for calling `nuit_drop_string` on this string!**
    render_json: extern "C" fn(*const CRoot) -> *const c_char,
    /// Fires an to the given JSON-serialized id path with the given JSON-serialized event.
    fire_event_json: extern "C" fn(*const CRoot, *const c_char, *const c_char),
    /// Registers a callback that we (the Rust side) can use to trigger UI updates.
    set_update_callback: extern "C" fn(*const CRoot, extern "C" fn()),
}

extern "C" fn render_json_impl<T>(c_root: *const CRoot) -> *const c_char where T: View {
    unsafe {
        let root = (*c_root).wrapped as *mut Root<T>;
        let json = (*root).render_json();
        let c_string = CString::new(json).expect("Could not convert JSON to C string");
        c_string.into_raw()
    }
}

extern "C" fn fire_event_json_impl<T>(c_root: *const CRoot, raw_id_path_json: *const c_char, raw_event_json: *const c_char) where T: View {
    unsafe {
        let root = (*c_root).wrapped as *mut Root<T>;
        let id_path_json = str::from_utf8(CStr::from_ptr(raw_id_path_json).to_bytes()).expect("Could not decode id path JSON");
        let event_json = str::from_utf8(CStr::from_ptr(raw_event_json).to_bytes()).expect("Could not decode event JSON");
        (*root).fire_event_json(id_path_json, event_json)
    }
}

extern "C" fn set_update_callback_impl<T>(c_root: *const CRoot, update_callback: extern "C" fn()) where T: View {
    unsafe {
        let root = (*c_root).wrapped as *mut Root<T>;
        (*root).set_update_callback(move || update_callback());
    }
}

impl<T> From<Box<Root<T>>> for CRoot where T: View {
    fn from(value: Box<Root<T>>) -> Self {
        Self {
            wrapped: Box::into_raw(value) as *mut c_void,
            render_json: render_json_impl::<T>,
            fire_event_json: fire_event_json_impl::<T>,
            set_update_callback: set_update_callback_impl::<T>,
        }
    }
}

impl Drop for CRoot {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.wrapped as *mut c_void));
        }
    }
}
