use std::{error::Error, fmt};

#[derive(Debug)]
pub enum EnvironmentError {
    Variable(std::env::VarError),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(error) => write!(formatter, "environment variable error: {error}"),
            Self::Io(error) => write!(formatter, "I/O error: {error}"),
            Self::Json(error) => write!(formatter, "JSON serialization error: {error}"),
        }
    }
}

impl Error for EnvironmentError {}

impl From<std::env::VarError> for EnvironmentError {
    fn from(value: std::env::VarError) -> Self {
        Self::Variable(value)
    }
}

impl From<std::io::Error> for EnvironmentError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for EnvironmentError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
