mod oidc;
mod pkce;
mod port;
mod string;
mod tokio_rt;
mod uuid;

use std::pin::Pin;

use crate::oidc::browser_oidc;
use crate::pkce::pkce_generate;
use crate::port::free_local_ipv4_port;
use crate::string::free_rust_string;
use crate::uuid::fill_uuid_v7_into_guid;

#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    #[namespace = "dwebble_cxx::oidc"]
    pub struct FPkce {
        pub code_verifier: String,
        pub code_challenge: String,
        pub code_challenge_method: String, // "S256" / "plain"
        pub state: String,
        pub nonce: String,
    }

    #[derive(Debug)]
    #[namespace = "dwebble_cxx::oidc"]
    pub struct FOidcResult {
        pub success: bool,
        pub access_token: String,
        pub refresh_token: String,
        pub error_message: String,
    }

    unsafe extern "C++" {
        // include!("Dwebble/Tokio/ITokioRuntimeProvider.h");
        include!("Dwebble/Bridge/DwebbleBridge.h");
        // type ITokioRuntimeProvider;
        type RustFutureString = crate::RustFutureString;
        type RustFutureOidcResult = crate::oidc::RustFutureOidcResult;
    }

    extern "Rust" {
        #[namespace = "dwebble_cxx::pkce"]
        fn pkce_generate() -> FPkce;

        #[namespace = "dwebble_cxx::port"]
        fn free_local_ipv4_port() -> Result<u16>;

        #[namespace = "dwebble_cxx::string"]
        unsafe fn free_rust_string(s: *mut c_char);

        #[namespace = "dwebble_cxx::uuid"]
        fn fill_uuid_v7_into_guid(buf: &mut [u8; 16]);

        #[namespace = "dwebble_cxx::oidc"]
        fn browser_oidc(
            issuer: String,
            client_id: String,
            client_secret: String,
            loopback_port: i32,
            loopback_route: String,
        ) -> RustFutureOidcResult;
    }
}

#[cxx_async::bridge]
unsafe impl Future for RustFutureString {
    type Output = String;
}
