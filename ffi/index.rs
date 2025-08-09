#[path = "../src/string.rs"]
pub mod string;
#[path = "../src/uuid.rs"]
pub mod uuid;

use crate::string::free_rust_string;
use crate::uuid::fill_uuid_v7_into_guid;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        #[namespace = "dwebble_cxx::uuid"]
        fn fill_uuid_v7_into_guid(buf: &mut [u8; 16]);

        #[namespace = "dwebble_cxx::string"]
        unsafe fn free_rust_string(s: *mut c_char);
    }
}
