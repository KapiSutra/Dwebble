use crate::{ffi::FOidcResult, tokio_rt};
use axum::{Router, extract::Query, routing::get};
use openidconnect::core::*;
use openidconnect::{
    AuthenticationFlow, AuthorizationCode, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce,
    OAuth2TokenResponse, PkceCodeChallenge, RedirectUrl, Scope, reqwest,
};
use std::i32;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

pub async fn oidc_token(
    issuer: &str,
    client_id: &str,
    client_secret: Option<&str>,
    loopback_port: i32,
    loopback_route: &str,
) -> anyhow::Result<CoreTokenResponse> {
    let http_client = reqwest::ClientBuilder::new()
        .build()
        .expect("Client build error");

    let provider =
        CoreProviderMetadata::discover_async(IssuerUrl::new(issuer.to_string())?, &http_client)
            .await?;

    let client = CoreClient::from_provider_metadata(
        provider,
        ClientId::new(client_id.to_string()),
        client_secret.map(|s| ClientSecret::new(s.to_string())),
    )
    .set_redirect_uri(RedirectUrl::new(format!(
        "http://127.0.0.1:{}{}",
        loopback_port, loopback_route
    ))?);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, state, _nonce) = client
        .authorize_url(
            AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
            CsrfToken::new_random,
            Nonce::new_random,
        )
        .set_pkce_challenge(pkce_challenge)
        .add_scope(Scope::new("openid".into()))
        .url();

    webbrowser::open(auth_url.as_str())?;

    #[derive(serde::Deserialize)]
    struct Params {
        code: String,
        state: String,
    }

    let (tx, rx) = oneshot::channel::<Params>();
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let arc_tx = Arc::new(Mutex::new(Some(tx)));
    let arc_shutdown_tx = Arc::new(Mutex::new(Some(shutdown_tx)));

    let expected_state = state.secret().to_string();
    let app = Router::new().route(
        &loopback_route,
        get(move |Query(p): Query<Params>| {
            let shared_tx_clone = arc_tx.clone();
            let expected_state_clone = expected_state.clone();
            let shared_shutdown_tx_clone = arc_shutdown_tx.clone();

            async move {
                assert_eq!(p.state, expected_state_clone);

                if p.state != expected_state_clone {
                    return "State does not match.";
                }

                if let Some(sender) = shared_tx_clone.lock().await.take() {
                    let _ = sender.send(p);

                    if let Some(shutdown) = shared_shutdown_tx_clone.lock().await.take() {
                        let _ = shutdown.send(());
                    }
                    "You can close this window."
                } else {
                    "This request has already been processed."
                }
            }
        }),
    );

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", loopback_port)).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        })
        .await?;

    let params = rx.await?;
    let code = params.code;

    let token_response = client
        .exchange_code(AuthorizationCode::new(code))?
        .set_pkce_verifier(pkce_verifier)
        .request_async(&http_client)
        .await?;

    Ok(token_response)
}

#[cxx_async::bridge]
unsafe impl Future for RustFutureOidcResult {
    type Output = FOidcResult;
}

pub fn browser_oidc(
    issuer: String,
    client_id: String,
    client_secret: String,
    loopback_port: i32,
    loopback_route: String,
) -> RustFutureOidcResult {
    RustFutureOidcResult::infallible(async move {
        let rt = tokio_rt::get_or_init_runtime();
        rt.spawn(async move {
            let result = oidc_token(
                issuer.as_str(),
                client_id.as_str(),
                // Only provide the secret if the string is not empty.
                (!client_secret.trim().is_empty()).then_some(client_secret.as_str()),
                loopback_port,
                loopback_route.as_str(),
            )
            .await;

            match result {
                Ok(token_response) => FOidcResult {
                    success: true,
                    access_token: token_response.access_token().secret().to_string(),
                    refresh_token: token_response
                        .refresh_token()
                        .map_or(String::new(), |rt| rt.secret().to_string()),
                    error_message: String::new(),
                },
                Err(e) => FOidcResult {
                    success: false,
                    access_token: String::new(),
                    refresh_token: String::new(),
                    error_message: e.to_string(),
                },
            }
        })
        .await
        .unwrap_or_else(|join_error| {
            // This block is executed only if the spawned task panicked.
            FOidcResult {
                success: false,
                access_token: String::new(),
                refresh_token: String::new(),
                error_message: format!("Task panicked: {}", join_error),
            }
        })
    })
}
