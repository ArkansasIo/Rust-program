use super::errors::EnvironmentError;
use std::ffi::{OsStr, OsString};

pub fn get(key: &str) -> Result<String, EnvironmentError> {
    Ok(std::env::var(key)?)
}

pub fn get_os(key: &OsStr) -> Option<OsString> {
    std::env::var_os(key)
}

pub fn exists(key: &str) -> bool {
    std::env::var_os(key).is_some()
}

pub fn all() -> Vec<(String, String)> {
    std::env::vars().collect()
}
