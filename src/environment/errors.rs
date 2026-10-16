use std::{error::Error, fmt};

#[derive(Debug)]
pub enum EnvironmentError {
    Variable(std::env::VarError),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(err) => write!(f, "environment variable error: {err}"),
            Self::Io(err) => write!(f, "I/O error: {err}"),
            Self::Json(err) => write!(f, "JSON serialization error: {err}"),
        }
    }
}

impl Error for EnvironmentError {}

impl From<std::env::VarError> for EnvironmentError {
    fn from(value: std::env::VarError) -> Self { Self::Variable(value) }
}

impl From<std::io::Error> for EnvironmentError {
    fn from(value: std::io::Error) -> Self { Self::Io(value) }
}

impl From<serde_json::Error> for EnvironmentError {
    fn from(value: serde_json::Error) -> Self { Self::Json(value) }
}
