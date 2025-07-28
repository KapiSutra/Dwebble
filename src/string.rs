pub mod string {
    use std::ffi::{CString, c_char};

    /// Frees memory allocated by Rust for strings passed to C++.
    /// This MUST be called from C++ when the string is no longer needed.
    #[unsafe(no_mangle)]
    pub unsafe extern "C" fn free_rust_string(s: *mut c_char) {
        unsafe {
            if s.is_null() {
                return;
            }
            // Re-take ownership of the CString from the raw pointer.
            // When `_` goes out of scope, the memory will be deallocated.
            let _ = CString::from_raw(s);
        }
    }
}
