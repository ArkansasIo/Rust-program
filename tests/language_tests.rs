use rust_environment_api::language::{evaluate, Value};
use std::collections::HashMap;

#[test]
fn expression_language_handles_arithmetic_and_logic() {
    let vars = HashMap::from([
        ("hp".to_owned(), Value::Integer(100)),
        ("alive".to_owned(), Value::Boolean(true)),
    ]);
    assert_eq!(evaluate("hp - 25 * 2", &vars), Ok(Value::Integer(50)));
    assert_eq!(evaluate("hp > 0 && alive", &vars), Ok(Value::Boolean(true)));
}

#[test]
fn expression_language_rejects_unsafe_or_ambiguous_cases() {
    let vars = HashMap::new();
    assert!(evaluate("1 / 3", &vars).is_err());
    assert!(evaluate("unknown", &vars).is_err());
    assert!(evaluate("true + 1", &vars).is_err());
}
