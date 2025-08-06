use std::ffi::c_void;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fill_uuid_v7_into_guid(buffer: *mut c_void) {
    unsafe {
        if buffer.is_null() {
            return;
        }

        let new_id = uuid::Uuid::now_v7();

        let uuid_bytes: uuid::Bytes = new_id.into_bytes(); // 这是标准的 UUID 大端字节数组

        let dest_bytes_ptr: *mut u8 = buffer as *mut u8;

        fn reverse_four_bytes(input: &[u8]) -> [u8; 4] {
            [input[3], input[2], input[1], input[0]]
        }
        // 对每个 4 字节块进行反转，然后复制。
        // 这样做的目的是让FGuid的A,B,C,D成员的值在小端系统上，
        // 能正确反映UUID标准的大端逻辑值。
        let reversed_a = reverse_four_bytes(&uuid_bytes[0..4]);
        std::ptr::copy_nonoverlapping(reversed_a.as_ptr(), dest_bytes_ptr.add(0), 4);

        let reversed_b = reverse_four_bytes(&uuid_bytes[4..8]);
        std::ptr::copy_nonoverlapping(reversed_b.as_ptr(), dest_bytes_ptr.add(4), 4);

        let reversed_c = reverse_four_bytes(&uuid_bytes[8..12]);
        std::ptr::copy_nonoverlapping(reversed_c.as_ptr(), dest_bytes_ptr.add(8), 4);

        let reversed_d = reverse_four_bytes(&uuid_bytes[12..16]);
        std::ptr::copy_nonoverlapping(reversed_d.as_ptr(), dest_bytes_ptr.add(12), 4);
    }
}
