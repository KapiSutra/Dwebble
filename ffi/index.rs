mod oidc;
mod pkce;
mod port;
mod string;
mod tokio_rt;
mod uuid;

use std::pin::Pin;

use crate::pkce::pkce_generate;
use crate::port::free_local_ipv4_port;
use crate::string::free_rust_string;
use crate::uuid::fill_uuid_v7_into_guid;

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    pub struct Pkce {
        pub code_verifier: String,
        pub code_challenge: String,
        pub code_challenge_method: String, // "S256" / "plain"
        pub state: String,
        pub nonce: String,
    }

    unsafe extern "C++" {
        include!("Dwebble/Tokio/ITokioRuntimeProvider.h");
        include!("Dwebble/Oidc/DwebbleOidcBridge.h");
        type ITokioRuntimeProvider;
        type RustFutureString = crate::RustFutureString;
    }

    extern "Rust" {
        #[namespace = "dwebble_cxx::pkce"]
        fn pkce_generate() -> Pkce;

        #[namespace = "dwebble_cxx::port"]
        fn free_local_ipv4_port() -> Result<u16>;

        #[namespace = "dwebble_cxx::string"]
        unsafe fn free_rust_string(s: *mut c_char);

        #[namespace = "dwebble_cxx::uuid"]
        fn fill_uuid_v7_into_guid(buf: &mut [u8; 16]);

        #[namespace = "dwebble_cxx::oidc"]
        fn oidc_access_token(
            issuer: String,
            client_id: String,
            client_secret: String,
            loopback_port: i32,
            loopback_route: String,
        ) -> RustFutureString;
    }
}

#[cxx_async::bridge]
unsafe impl Future for RustFutureString {
    type Output = String;
}

fn oidc_access_token(
    issuer: String,
    client_id: String,
    client_secret: String,
    loopback_port: i32,
    loopback_route: String,
) -> RustFutureString {
    RustFutureString::infallible(async move {
        let rt = tokio_rt::get_or_init_runtime();
        rt.spawn(async move {
            let result = oidc::oidc_access_token(
                issuer,
                client_id,
                (!client_secret.trim().is_empty()).then_some(client_secret),
                loopback_port,
                loopback_route,
            )
            .await;

            result.unwrap_or_else(|_e| String::new())
        })
        .await
        .unwrap_or_else(|_| String::new())
    })
}
