use super::errors::EnvironmentError;
use std::path::PathBuf;

pub fn current_dir() -> Result<PathBuf, EnvironmentError> {
    Ok(std::env::current_dir()?)
}

pub fn executable() -> Result<PathBuf, EnvironmentError> {
    Ok(std::env::current_exe()?)
}

pub fn temp_dir() -> PathBuf {
    std::env::temp_dir()
}
