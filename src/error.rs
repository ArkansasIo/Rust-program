use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    InvalidInput(String),
    NotFound(String),
    AlreadyExists(String),
    NotAllowed(String),
    CapacityExceeded(String),
    Conflict(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(v) => write!(f, "invalid input: {v}"),
            Self::NotFound(v) => write!(f, "not found: {v}"),
            Self::AlreadyExists(v) => write!(f, "already exists: {v}"),
            Self::NotAllowed(v) => write!(f, "not allowed: {v}"),
            Self::CapacityExceeded(v) => write!(f, "capacity exceeded: {v}"),
            Self::Conflict(v) => write!(f, "conflict: {v}"),
        }
    }
}

impl std::error::Error for EngineError {}
pub type EngineResult<T> = Result<T, EngineError>;
