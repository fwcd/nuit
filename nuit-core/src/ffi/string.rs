use std::ffi::{CString, c_char};

#[no_mangle]
pub extern "C" fn nuit_c_string_drop(raw_string: *const c_char) {
    unsafe {
        drop(CString::from_raw(raw_string.cast_mut()));
    }
}
