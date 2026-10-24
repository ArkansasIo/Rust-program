use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub language: String,
    pub theme: String,
    pub notifications: bool,
    pub telemetry: bool,
    pub developer_mode: bool,
    pub api_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: "en".into(),
            theme: "system".into(),
            notifications: true,
            telemetry: false,
            developer_mode: false,
            api_enabled: true,
        }
    }
}
