#[path = "../src/port.rs"]
pub mod port;
#[path = "../src/string.rs"]
pub mod string;
#[path = "../src/uuid.rs"]
pub mod uuid;

#[path = "../src/pkce.rs"]
pub mod pkce;

use crate::pkce::pkce_generate;
use crate::port::free_local_ipv4_port;
use crate::string::free_rust_string;
use crate::uuid::fill_uuid_v7_into_guid;

#[cxx::bridge]
mod ffi {

    #[derive(Debug)]
    pub struct Pkce {
        pub code_verifier: String,
        pub code_challenge: String,
        pub code_challenge_method: String, // "S256" / "plain"
        pub state: String,
        pub nonce: String,
    }

    extern "Rust" {
        #[namespace = "dwebble_cxx::uuid"]
        fn fill_uuid_v7_into_guid(buf: &mut [u8; 16]);

        #[namespace = "dwebble_cxx::string"]
        unsafe fn free_rust_string(s: *mut c_char);

        #[namespace = "dwebble_cxx::port"]
        fn free_local_ipv4_port() -> Result<u16>;

        #[namespace = "dwebble_cxx::pkce"]
        fn pkce_generate() -> Pkce;
    }
}
