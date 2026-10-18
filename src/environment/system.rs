use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SystemInfo {
    pub os: String,
    pub family: String,
    pub architecture: String,
    pub executable: String,
    pub process_id: u32,
    pub package_version: String,
    pub rust_version: String,
}

pub fn current() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        family: std::env::consts::FAMILY.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        executable: std::env::current_exe()
            .map(|path| path.display().to_string())
            .unwrap_or_default(),
        process_id: std::process::id(),
        package_version: env!("CARGO_PKG_VERSION").to_string(),
        rust_version: option_env!("RUSTC_VERSION")
            .unwrap_or("unknown")
            .to_string(),
    }
}
