use std::{env, error::Error, fmt, io};

#[derive(Debug)]
pub enum EnvironmentError {
    Variable(env::VarError),
    Io(io::Error),
    Json(serde_json::Error),
}

impl fmt::Display for EnvironmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(err) => write!(f, "Environment variable error: {err}"),
            Self::Io(err) => write!(f, "Environment I/O error: {err}"),
            Self::Json(err) => write!(f, "JSON serialization error: {err}"),
        }
    }
}

impl Error for EnvironmentError {}

impl From<env::VarError> for EnvironmentError {
    fn from(value: env::VarError) -> Self {
        Self::Variable(value)
    }
}

impl From<io::Error> for EnvironmentError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<serde_json::Error> for EnvironmentError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}
