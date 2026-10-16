use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub roles: Vec<String>,
    pub enabled: bool,
}

impl Account {
    pub fn new(id: impl Into<String>, username: impl Into<String>, email: impl Into<String>) -> Self {
        let username = username.into();
        Self { id: id.into(), display_name: username.clone(), username, email: email.into(), roles: vec!["user".into()], enabled: true }
    }

    pub fn is_admin(&self) -> bool { self.roles.iter().any(|r| r == "admin") }
}
