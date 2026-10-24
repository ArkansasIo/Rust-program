//! Small, dependency-light mathematics helpers for the Rust Environment API.
use serde::Serialize;

/// Classification of an integer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct IntegerClassification { pub value: i64, pub is_even: bool, pub is_odd: bool, pub is_prime: bool }

/// Return whether an integer is prime. Values below two are not prime.
pub fn is_prime(value: i64) -> bool {
    if value < 2 { return false; }
    if value == 2 { return true; }
    if value % 2 == 0 { return false; }
    let mut divisor = 3_i64;
    while divisor <= value / divisor {
        if value % divisor == 0 { return false; }
        divisor += 2;
    }
    true
}

/// Classify parity and primality.
pub fn classify_integer(value: i64) -> IntegerClassification {
    let is_even = value % 2 == 0;
    IntegerClassification { value, is_even, is_odd: !is_even, is_prime: is_prime(value) }
}

/// Euclid's greatest common divisor. Returns None when abs(i64::MIN) is required.
pub fn gcd(a: i64, b: i64) -> Option<u64> {
    let mut x = a.checked_abs()? as u64;
    let mut y = b.checked_abs()? as u64;
    while y != 0 { let remainder = x % y; x = y; y = remainder; }
    Some(x)
}

/// Checked integer arithmetic. None indicates invalid operation, overflow,
/// division by zero, or a non-integral division.
pub fn checked_integer_operation(op: &str, a: i64, b: i64) -> Option<i64> {
    match op {
        "add" | "+" => a.checked_add(b),
        "sub" | "-" => a.checked_sub(b),
        "mul" | "*" => a.checked_mul(b),
        "div" | "/" if b != 0 && a % b == 0 => a.checked_div(b),
        "div" | "/" => None,
        "mod" | "%" if b != 0 => a.checked_rem(b),
        "mod" | "%" => None,
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn prime_and_parity_edges() {
        assert!(!is_prime(i64::MIN)); assert!(!is_prime(-7)); assert!(!is_prime(0));
        assert!(!is_prime(1)); assert!(is_prime(2)); assert!(is_prime(97)); assert!(!is_prime(99));
        assert!(classify_integer(8).is_even); assert!(classify_integer(9).is_odd);
    }
    #[test] fn gcd_handles_zero_and_negative_values() {
        assert_eq!(gcd(54, 24), Some(6)); assert_eq!(gcd(-54, 24), Some(6));
        assert_eq!(gcd(0, 7), Some(7)); assert_eq!(gcd(0, 0), Some(0));
        assert_eq!(gcd(i64::MIN, 1), None);
    }
    #[test] fn checked_arithmetic_rejects_invalid_results() {
        assert_eq!(checked_integer_operation("add", 2, 3), Some(5));
        assert_eq!(checked_integer_operation("*", 7, 6), Some(42));
        assert_eq!(checked_integer_operation("div", 9, 2), None);
        assert_eq!(checked_integer_operation("div", 9, 3), Some(3));
        assert_eq!(checked_integer_operation("div", 1, 0), None);
        assert_eq!(checked_integer_operation("add", i64::MAX, 1), None);
        assert_eq!(checked_integer_operation("pow", 2, 3), None);
    }
}