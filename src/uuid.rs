use std::ffi::{CString, c_char};
use uuid;

#[unsafe(no_mangle)]
pub extern "C" fn fill_uuid_v7_into_guid(buf: &mut [u8; 16]) -> *mut c_char {
    let new_id = uuid::Uuid::now_v7();

    let uuid_bytes: uuid::Bytes = new_id.into_bytes();

    fn reverse_four_bytes(input: &[u8]) -> [u8; 4] {
        [input[3], input[2], input[1], input[0]]
    }

    buf[0..4].copy_from_slice(&reverse_four_bytes(&uuid_bytes[0..4]));
    buf[4..8].copy_from_slice(&reverse_four_bytes(&uuid_bytes[4..8]));
    buf[8..12].copy_from_slice(&reverse_four_bytes(&uuid_bytes[8..12]));
    buf[12..16].copy_from_slice(&reverse_four_bytes(&uuid_bytes[12..16]));

    CString::new(new_id.to_string()).unwrap().into_raw()
}
