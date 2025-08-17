mod oidc;
mod pkce;
mod port;
mod string;
mod tokio_rt;
mod uuid;

use std::pin::Pin;

use openidconnect::OAuth2TokenResponse;

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
        // pub refresh_token: String,
        pub error_message: String,
    }

    unsafe extern "C++" {
        // include!("Dwebble/Tokio/ITokioRuntimeProvider.h");
        include!("Dwebble/Bridge/DwebbleBridge.h");
        // type ITokioRuntimeProvider;
        type RustFutureString = crate::RustFutureString;
        type RustFutureOidcResult = crate::RustFutureOidcResult;
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

#[cxx_async::bridge]
unsafe impl Future for RustFutureOidcResult {
    type Output = ffi::FOidcResult;
}

fn browser_oidc(
    issuer: String,
    client_id: String,
    client_secret: String,
    loopback_port: i32,
    loopback_route: String,
) -> RustFutureOidcResult {
    RustFutureOidcResult::infallible(async move {
        let rt = tokio_rt::get_or_init_runtime();
        rt.spawn(async move {
            let result = oidc::oidc_access_token(
                issuer,
                client_id,
                // Only provide the secret if the string is not empty.
                (!client_secret.trim().is_empty()).then_some(client_secret),
                loopback_port,
                loopback_route,
            )
            .await;

            // 2. 正确处理 Result
            match result {
                // 成功时，提取 access_token
                Ok(token_response) => ffi::FOidcResult {
                    success: true,
                    access_token: token_response.access_token().secret().to_string(),
                    error_message: String::new(),
                },
                // 失败时，将错误格式化为字符串
                Err(_e) => ffi::FOidcResult {
                    success: false,
                    access_token: String::new(),
                    error_message: _e.to_string(),
                },
            }
        })
        .await
        // 3. 处理任务本身可能发生的 panic
        .unwrap_or(ffi::FOidcResult {
            success: false,
            access_token: "".to_string(),
            error_message: "".to_string(),
        })
    })
}
