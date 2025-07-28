// #[cxx::bridge(namespace = "dwebble")]
// pub mod ffi {
//     extern "Rust" {
//         pub fn test() -> u8;
//     }
//
//     unsafe extern "C++" {}
// }

use std::ffi::c_void;
use uuid;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fill_uuid_v7_into_buffer(buffer: *mut c_void) {
    unsafe {
        if buffer.is_null() {
            return;
        }

        let uuid_bytes: uuid::Bytes = uuid::Uuid::now_v7().into_bytes();

        let dest_ptr: *mut [u8; 16] = buffer as *mut [u8; 16];

        std::ptr::copy_nonoverlapping(uuid_bytes.as_ptr(), dest_ptr as *mut u8, 16);
    }
}
