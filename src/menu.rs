use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub command: String,
    pub enabled: bool,
    pub children: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(id: &str, label: &str, command: &str) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            command: command.into(),
            enabled: true,
            children: Vec::new(),
        }
    }

    pub fn with_children(mut self, children: Vec<MenuItem>) -> Self {
        self.children = children;
        self
    }
}

pub fn default_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::new("home", "Home", "home"),
        MenuItem::new("account", "Account", "account").with_children(vec![
            MenuItem::new("profile", "Profile", "account.profile"),
            MenuItem::new("security", "Security", "account.security"),
            MenuItem::new("sessions", "Sessions", "account.sessions"),
            MenuItem::new(
                "api-applications",
                "API Applications",
                "account.api-applications",
            ),
        ]),
        MenuItem::new("math", "Mathematics", "math").with_children(vec![
            MenuItem::new("calculator", "Calculator", "math.calculate"),
            MenuItem::new("algebra", "Algebra", "math.algebra"),
            MenuItem::new("calculus", "Calculus", "math.calculus"),
            MenuItem::new("equations", "Equations", "math.equations"),
            MenuItem::new("logic", "Logic", "math.logic"),
            MenuItem::new("linear-algebra", "Linear Algebra", "math.linear-algebra"),
            MenuItem::new("statistics", "Statistics", "math.statistics"),
            MenuItem::new("units", "Units", "math.units"),
        ]),
        MenuItem::new("environment", "Environment", "environment").with_children(vec![
            MenuItem::new("system", "System", "environment.system"),
            MenuItem::new("variables", "Variables", "environment.variables"),
            MenuItem::new("process", "Process", "environment.process"),
            MenuItem::new("snapshot", "Snapshot", "environment.snapshot"),
        ]),
        MenuItem::new("settings", "Settings", "settings").with_children(vec![
            MenuItem::new("general", "General", "settings.general"),
            MenuItem::new("appearance", "Appearance", "settings.appearance"),
            MenuItem::new("privacy", "Privacy", "settings.privacy"),
            MenuItem::new("notifications", "Notifications", "settings.notifications"),
            MenuItem::new("developer", "Developer", "settings.developer"),
        ]),
        MenuItem::new("help", "Help", "help").with_children(vec![
            MenuItem::new("documentation", "Documentation", "help.documentation"),
            MenuItem::new("api", "API Reference", "help.api"),
            MenuItem::new("about", "About", "help.about"),
        ]),
    ]
}
