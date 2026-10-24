use rust_environment_api::math::{checked_integer_operation, classify_integer, gcd, is_prime};

#[test]
fn integer_classifier_reports_prime_parity() {
    let result = classify_integer(13);
    assert!(result.is_odd);
    assert!(!result.is_even);
    assert!(result.is_prime);
    let composite = classify_integer(15);
    assert!(composite.is_odd);
    assert!(!composite.is_prime);
}

#[test]
fn arithmetic_is_checked() {
    assert_eq!(checked_integer_operation("add", 10, 2), Some(12));
    assert_eq!(checked_integer_operation("sub", 10, 2), Some(8));
    assert_eq!(checked_integer_operation("mul", 10, 2), Some(20));
    assert_eq!(checked_integer_operation("div", 10, 2), Some(5));
    assert_eq!(checked_integer_operation("mod", 10, 3), Some(1));
    assert_eq!(checked_integer_operation("div", 10, 3), None);
}

#[test]
fn euclidean_gcd_is_available() {
    assert_eq!(gcd(18, 30), Some(6));
    assert!(is_prime(2));
    assert!(!is_prime(1));
}
