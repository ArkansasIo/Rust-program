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
    let key = "RUST_ENVIRONMENT_API_TEST";
    unsafe { std::env::set_var(key, "ok"); }
    assert_eq!(Environment::get(key).unwrap(), "ok");
    unsafe { std::env::remove_var(key); }
}

#[test]
fn snapshot_serializes() {
    let snapshot = Environment::snapshot().expect("snapshot should collect");
    let json = snapshot.to_json_pretty().expect("snapshot should serialize");
    assert!(json.contains("current_directory"));
}
