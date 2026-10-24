# Mathematics module

This module provides deterministic integer helpers. It is a foundation for larger
math APIs, not a symbolic algebra or computer algebra system.

## Rust API

```rust
use rust_environment_api::math::{checked_integer_operation, classify_integer, gcd, is_prime};

assert!(is_prime(97));
assert!(classify_integer(12).is_even);
assert_eq!(gcd(54, 24), Some(6));
assert_eq!(checked_integer_operation("mul", 6, 7), Some(42));
```

## CLI

- `rust-env math prime 97` tests primality.
- `rust-env math classify 42` reports parity and primality as JSON.
- `rust-env math gcd 54 24` calculates the greatest common divisor.
- `rust-env math calc mul 6 7` runs checked integer arithmetic.

Operators: `add`, `sub`, `mul`, `div`, `mod` (also accepts common symbols).
Division is integer-only: non-integral results are errors, not silently truncated.
Overflow, division by zero, and unknown operators are errors.

## Edge cases

- Integers less than two are not prime.
- Negative integers are not prime.
- `gcd(0, 0)` is defined here as zero.
- `gcd` returns `None` if either input is `i64::MIN`, whose absolute magnitude cannot fit in `i64`.
- Checked operations return `None` for invalid operations or results.

Run `cargo test` to run tests.
