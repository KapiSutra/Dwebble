use pkce_std::{Code, Count, encoding, generate};

pub fn pkce_generate() -> crate::ffi::Pkce {
    let code = Code::generate_default();

    let (verifier, challenge) = code.into_pair();

    // 32 字节强随机 → base64url（无填充）作为 state/nonce
    let cnt = Count::new(32).unwrap_or(Count::DEFAULT);
    let state = encoding::encode(generate::bytes(cnt));
    let nonce = encoding::encode(generate::bytes(cnt));

    crate::ffi::Pkce {
        code_verifier: verifier.to_string(),
        code_challenge: challenge.to_string(),
        code_challenge_method: challenge.method().static_str().to_string(),
        state,
        nonce,
    }
}
