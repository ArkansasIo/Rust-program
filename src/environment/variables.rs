use std::ffi::{OsStr, OsString};

use super::errors::EnvironmentError;

/// Marker used when an environment variable's name suggests it may hold a secret.
pub const REDACTED_VALUE: &str = "[REDACTED]";

pub fn get(key: &str) -> Result<String, EnvironmentError> {
    Ok(std::env::var(key)?)
}

pub fn get_os(key: &OsStr) -> Option<OsString> {
    std::env::var_os(key)
}

pub fn exists(key: &str) -> bool {
    std::env::var_os(key).is_some()
}

/// Return environment variables with likely credentials redacted by default.
///
/// This heuristic relies on names and cannot guarantee that all secrets are detected.
pub fn all() -> Vec<(String, String)> {
    std::env::vars()
        .map(|(key, value)| {
            let value = if is_sensitive_name(&key) {
                REDACTED_VALUE.to_string()
            } else {
                value
            };
            (key, value)
        })
        .collect()
}

/// Return all Unicode environment variables without redaction.
///
/// Use only for trusted local diagnostics; values may contain credentials.
pub fn all_unredacted() -> Vec<(String, String)> {
    std::env::vars().collect()
}

/// Heuristically identify names that commonly contain credentials or secrets.
pub fn is_sensitive_name(name: &str) -> bool {
    let normalized = name.to_ascii_uppercase();
    let compact: String = normalized
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .collect();

    const SECRET_MARKERS: &[&str] = &[
        "SECRET",
        "TOKEN",
        "PASSWORD",
        "PASSWD",
        "CREDENTIAL",
        "PRIVATEKEY",
        "APIKEY",
        "ACCESSKEY",
        "CLIENTSECRET",
        "AUTHORIZATION",
        "BEARER",
        "SESSIONKEY",
        "SIGNINGKEY",
        "COOKIE",
    ];

    SECRET_MARKERS.iter().any(|marker| compact.contains(marker)) || compact.ends_with("KEY")
}
