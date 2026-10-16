use serde::Serialize;
use super::{errors::EnvironmentError, paths, system::{self, SystemInfo}, variables};

#[derive(Debug, Clone, Serialize)]
pub struct EnvironmentSnapshot {
    pub system: SystemInfo,
    pub arguments: Vec<String>,
    pub current_directory: String,
    pub executable: String,
    pub temp_directory: String,
    pub variables: Vec<(String, String)>,
}

impl EnvironmentSnapshot {
    pub fn collect() -> Result<Self, EnvironmentError> {
        let current_directory = paths::current_dir()?;
        let executable = paths::executable()?;
        Ok(Self {
            system: system::current(),
            arguments: std::env::args_os().map(|v| v.to_string_lossy().into_owned()).collect(),
            current_directory: current_directory.display().to_string(),
            executable: executable.display().to_string(),
            temp_directory: paths::temp_dir().display().to_string(),
            variables: variables::all(),
        })
    }

    pub fn to_json_pretty(&self) -> Result<String, EnvironmentError> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
