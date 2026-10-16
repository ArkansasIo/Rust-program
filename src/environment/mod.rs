mod arguments;
mod errors;
mod paths;
mod snapshot;
mod system;
mod variables;

pub use arguments::ProcessArguments;
pub use errors::EnvironmentError;
pub use snapshot::EnvironmentSnapshot;
pub use system::SystemInfo;

/// Public entry point for inspecting the current Rust process environment.
pub struct Environment;

impl Environment {
    pub fn get(key: &str) -> Result<String, EnvironmentError> {
        variables::get(key)
    }

    pub fn get_os(key: &std::ffi::OsStr) -> Option<std::ffi::OsString> {
        variables::get_os(key)
    }

    pub fn exists(key: &str) -> bool {
        variables::exists(key)
    }

    pub fn variables() -> Vec<(String, String)> {
        variables::all()
    }

    pub fn arguments() -> ProcessArguments {
        arguments::current()
    }

    pub fn current_dir() -> Result<std::path::PathBuf, EnvironmentError> {
        paths::current_dir()
    }

    pub fn executable() -> Result<std::path::PathBuf, EnvironmentError> {
        paths::executable()
    }

    pub fn temp_dir() -> std::path::PathBuf {
        paths::temp_dir()
    }

    pub fn system() -> SystemInfo {
        system::current()
    }

    pub fn snapshot() -> Result<EnvironmentSnapshot, EnvironmentError> {
        EnvironmentSnapshot::collect()
    }
}
