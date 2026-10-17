use rust_environment_api::Environment;

#[test]
fn package_version_is_present() {
    assert!(!Environment::system().package_version.is_empty());
}

#[test]
fn executable_path_is_available() {
    let path = Environment::executable().expect("current executable should be available");
    assert!(path.exists());
}

#[test]
fn current_directory_is_available() {
    let path = Environment::current_dir().expect("current directory should be available");
    assert!(path.is_dir());
}

#[test]
fn environment_lookup_works() {
    const KEY: &str = "RUST_ENVIRONMENT_API_TEST";
    unsafe {
        std::env::set_var(KEY, "ok");
    }
    assert_eq!(Environment::get(KEY).unwrap(), "ok");
    unsafe {
        std::env::remove_var(KEY);
    }
}

#[test]
fn snapshot_serializes() {
    let snapshot = Environment::snapshot().expect("snapshot should collect");
    let json = snapshot.to_json_pretty().expect("snapshot should serialize");
    assert!(json.contains("current_directory"));
}

#[test]
fn redacts_common_secret_names() {
    for name in [
        "API_TOKEN",
        "GITHUB_TOKEN",
        "DATABASE_PASSWORD",
        "AWS_ACCESS_KEY_ID",
        "PRIVATE_KEY",
        "CLIENTSECRET",
        "AUTHORIZATION",
    ] {
        assert!(
            Environment::is_sensitive_variable_name(name),
            "expected {name} to be identified as sensitive"
        );
    }
}

#[test]
fn does_not_mark_plain_names_as_sensitive() {
    for name in ["PATH", "HOME", "LANG", "TERM", "RUST_LOG"] {
        assert!(
            !Environment::is_sensitive_variable_name(name),
            "unexpectedly marked {name} as sensitive"
        );
    }
}

#[test]
fn redacted_snapshot_replaces_sensitive_values() {
    const KEY: &str = "RUST_ENVIRONMENT_API_TEST_TOKEN";
    unsafe {
        std::env::set_var(KEY, "not-a-real-token");
    }

    let snapshot = Environment::snapshot().expect("snapshot should collect");
    let value = snapshot
        .variables
        .iter()
        .find(|(key, _)| key == KEY)
        .map(|(_, value)| value)
        .expect("test variable should be present");
    assert_eq!(value, "[REDACTED]");

    unsafe {
        std::env::remove_var(KEY);
    }
}

#[test]
fn raw_snapshot_keeps_sensitive_values() {
    const KEY: &str = "RUST_ENVIRONMENT_API_TEST_SECRET";
    unsafe {
        std::env::set_var(KEY, "local-test-secret");
    }

    let snapshot = Environment::snapshot_unredacted().expect("snapshot should collect");
    let value = snapshot
        .variables
        .iter()
        .find(|(key, _)| key == KEY)
        .map(|(_, value)| value)
        .expect("test variable should be present");
    assert_eq!(value, "local-test-secret");

    unsafe {
        std::env::remove_var(KEY);
    }
}
