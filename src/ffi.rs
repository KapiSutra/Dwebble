#[cxx::bridge(namespace = "dwebble")]
pub mod ffi {
    extern "Rust" {
        pub fn test() -> u8;
    }

    unsafe extern "C++" {}
}

pub fn test() -> u8 {
    return 22;
}
