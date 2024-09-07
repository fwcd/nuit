use std::{ffi::{c_void, c_char, CString, CStr}, str};

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
        (*root).fire_event_json(id_path_json, event_json)
    }
}

extern "C" fn set_update_callback_impl<T>(c_root: *const CRoot, update_callback: extern "C" fn(*const c_char)) where T: View {
    unsafe {
        let root = (*c_root).wrapped as *const Root<T>;
        (*root).set_update_callback(move |update| {
            let update_json = serde_json::to_string(update).expect("Could not encode update to JSON");
            let update_json_c_string = CString::new(update_json).expect("Could not convert update JSON to C string");
            update_callback(update_json_c_string.as_ptr())
        });
    }
}

impl CRoot {
    /// Creates a [`CRoot`] from the given boxed [`Root<T>`] for FFI.
    /// 
    /// # Ownership
    /// 
    /// Full ownership is transferred to the [`CRoot`], hence **this will leak
    /// the root unless converted back via [`CRoot::into_typed`]**, since the
    /// [`CRoot`] does not have a [`Drop`] implementation (due to type erasure
    /// making it impossible). For most uses this will likely not be an issue
    /// though since the root will likely be held for the entire duration of the
    /// application.
    pub fn from_typed<T>(root: Box<Root<T>>) -> Self where T: View {
        Self {
            wrapped: Box::into_raw(root) as *mut c_void,
            render_json: render_json_impl::<T>,
            fire_event_json: fire_event_json_impl::<T>,
            set_update_callback: set_update_callback_impl::<T>,
        }
    }

    /// Creates a `Root<T>` from a `CRoot`.
    /// 
    /// # Safety
    /// 
    /// It is the caller's responsibility to ensure that the same `T` is used as
    /// for the original instance, hence why this method is marked unsafe.
    pub unsafe fn into_typed<T>(self) -> Box<Root<T>> where T: View {
        Box::from_raw(self.wrapped as *mut Root<T>)
    }
}
