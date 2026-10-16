use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiApplication {
    pub id: String,
    pub name: String,
    pub description: String,
    pub owner_account_id: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
}

impl ApiApplication {
    pub fn new(id: &str, name: &str, owner_account_id: &str) -> Self {
        Self { id: id.into(), name: name.into(), description: String::new(), owner_account_id: owner_account_id.into(), scopes: vec!["read".into()], enabled: true }
    }

    pub fn can(&self, scope: &str) -> bool { self.enabled && self.scopes.iter().any(|s| s == scope || s == "*") }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiApplicationConfig {
    pub base_path: String,
    pub version: String,
    pub authentication: String,
    pub rate_limit_per_minute: u32,
    pub scopes: Vec<String>,
}

impl Default for ApiApplicationConfig {
    fn default() -> Self {
        Self { base_path: "/api".into(), version: "v1".into(), authentication: "bearer-token".into(), rate_limit_per_minute: 60, scopes: vec!["read".into(), "write".into(), "math".into(), "environment".into()] }
    }
}
