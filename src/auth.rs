use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

pub const SESSION_TTL_SECONDS: i64 = 60 * 60 * 24 * 180;

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| format!("hash password: {error}"))
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn generate_session_secret() -> String {
    let mut bytes = [0_u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

pub fn create_signed_session(secret: &str, now_unix: i64) -> String {
    let expires_at = now_unix + SESSION_TTL_SECONDS;
    let nonce = Uuid::new_v4();
    let payload = format!("v1.{expires_at}.{nonce}");
    let signature = sign_session(secret, &payload);
    format!("{payload}.{signature}")
}

pub fn validate_signed_session(secret: &str, token: &str, now_unix: i64) -> bool {
    let mut parts = token.split('.');
    let Some(version) = parts.next() else {
        return false;
    };
    let Some(expires_at_raw) = parts.next() else {
        return false;
    };
    let Some(nonce) = parts.next() else {
        return false;
    };
    let Some(signature) = parts.next() else {
        return false;
    };
    if parts.next().is_some() || version != "v1" || nonce.trim().is_empty() {
        return false;
    }

    let Ok(expires_at) = expires_at_raw.parse::<i64>() else {
        return false;
    };
    if expires_at <= now_unix {
        return false;
    }

    let payload = format!("{version}.{expires_at_raw}.{nonce}");
    let expected = sign_session(secret, &payload);
    constant_time_eq(expected.as_bytes(), signature.as_bytes())
}

fn sign_session(secret: &str, payload: &str) -> String {
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC accepts any key size");
    mac.update(b"ct-codex-bridge-session-v1");
    mac.update(payload.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0_u8, |diff, (left, right)| diff | (left ^ right))
        == 0
}

pub fn parse_cookie(header: Option<&str>, name: &str) -> Option<String> {
    let header = header?;
    header
        .split(';')
        .filter_map(|part| part.trim().split_once('='))
        .find_map(|(key, value)| {
            if key == name {
                Some(value.to_string())
            } else {
                None
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_verifies_without_storing_plaintext() {
        let hash = hash_password("quiet-control-room").unwrap();
        assert!(!hash.contains("quiet-control-room"));
        assert!(verify_password(&hash, "quiet-control-room"));
        assert!(!verify_password(&hash, "wrong"));
    }

    #[test]
    fn cookie_parser_finds_named_cookie() {
        assert_eq!(
            parse_cookie(
                Some("a=1; ct_codex_bridge=token-42; b=2"),
                "ct_codex_bridge"
            ),
            Some("token-42".to_string())
        );
    }

    #[test]
    fn signed_session_survives_process_restart_without_server_memory() {
        let secret = generate_session_secret();
        let token = create_signed_session(&secret, 1_700_000_000);
        assert!(validate_signed_session(&secret, &token, 1_700_000_100));
        assert!(!validate_signed_session(
            &secret,
            &token,
            1_700_000_000 + SESSION_TTL_SECONDS + 1
        ));
        assert!(!validate_signed_session(
            "wrong-secret",
            &token,
            1_700_000_100
        ));
    }
}
