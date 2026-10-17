use serde::Serialize;

use super::{
    errors::EnvironmentError,
    paths,
    system::{self, SystemInfo},
    variables,
};

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
    /// Collect a snapshot with likely secret values redacted.
    pub fn collect() -> Result<Self, EnvironmentError> {
        Self::collect_with_variables(variables::all())
    }

    /// Collect a snapshot without redacting environment variable values.
    ///
    /// The result may contain credentials and should not be logged or shared.
    pub fn collect_unredacted() -> Result<Self, EnvironmentError> {
        Self::collect_with_variables(variables::all_unredacted())
    }

    fn collect_with_variables(
        environment_variables: Vec<(String, String)>,
    ) -> Result<Self, EnvironmentError> {
        let current_directory = paths::current_dir()?;
        let executable = paths::executable()?;
        Ok(Self {
            system: system::current(),
            arguments: std::env::args_os()
                .map(|value| value.to_string_lossy().into_owned())
                .collect(),
            current_directory: current_directory.display().to_string(),
            executable: executable.display().to_string(),
            temp_directory: paths::temp_dir().display().to_string(),
            variables: environment_variables,
        })
    }

    pub fn to_json_pretty(&self) -> Result<String, EnvironmentError> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
