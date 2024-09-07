use std::{ffi::{c_char, c_void, CStr, CString}, ptr, str};

use crate::{Root, View};

/// A C/FFI-compatible wrapper around `Root<T>`.
#[repr(C)]
pub struct CRoot {
    /// The opaque pointer to the owned underlying Rust `Root<T>`.
    wrapped: *mut c_void,
    /// Renders the view to an owned JSON-serialized node tree.
    /// **Callers are responsible for calling `nuit_drop_string` on this string!**
    render_json: extern "C" fn(*const CRoot) -> *const c_char,
    /// Fires an to the given JSON-serialized id path with the given JSON-serialized event.
    fire_event_json: extern "C" fn(*const CRoot, *const c_char, *const c_char),
    /// Registers a callback that we (the Rust side) can use to trigger UI updates.
    set_update_callback: extern "C" fn(*const CRoot, extern "C" fn(*const c_char)),
}

extern "C" fn render_json_impl<T>(c_root: *const CRoot) -> *const c_char where T: View {
    unsafe {
        let root = (*c_root).wrapped as *const Root<T>;
        let json = (*root).render_json();
        let c_string = CString::new(json).expect("Could not convert JSON to C string");
        c_string.into_raw()
    }
}

extern "C" fn fire_event_json_impl<T>(c_root: *const CRoot, raw_id_path_json: *const c_char, raw_event_json: *const c_char) where T: View {
    unsafe {
        let root = (*c_root).wrapped as *const Root<T>;
        let id_path_json = str::from_utf8(CStr::from_ptr(raw_id_path_json).to_bytes()).expect("Could not decode id path JSON");
        let event_json = str::from_utf8(CStr::from_ptr(raw_event_json).to_bytes()).expect("Could not decode event JSON");
        (*root).fire_event_json(id_path_json, event_json);
    }
}

extern "C" fn set_update_callback_impl<T>(c_root: *const CRoot, update_callback: extern "C" fn(*const c_char)) where T: View {
    unsafe {
        let root = (*c_root).wrapped as *const Root<T>;
        (*root).set_update_callback(move |update| {
            let update_json = serde_json::to_string(update).expect("Could not encode update to JSON");
            let update_json_c_string = CString::new(update_json).expect("Could not convert update JSON to C string");
            update_callback(update_json_c_string.as_ptr());
        });
    }
}

impl CRoot {
    /// Safely uses a [`CRoot`] while returning ownership once the method returns.
    pub fn scope_from<T, U>(root: &mut Box<Root<T>>, action: impl FnOnce(&Self) -> U) -> U where T: View {
        let c_root = Self {
            wrapped: ptr::from_mut(&mut **root).cast::<c_void>(),
            render_json: render_json_impl::<T>,
            fire_event_json: fire_event_json_impl::<T>,
            set_update_callback: set_update_callback_impl::<T>,
        };

        action(&c_root)
    }
}
