// SPDX-License-Identifier: MPL-2.0
//! Integration tests: exercise the public API exactly as a consumer would.

use @@CRATE_NAME@@::{clamp, midpoint};

#[test]
fn clamp_is_idempotent() {
    for value in [0u32, 1, 5, 42, u32::MAX] {
        let once = clamp(value, 10, 100);
        assert_eq!(clamp(once, 10, 100), once, "clamping twice changed {value}");
    }
}

#[test]
fn clamp_result_always_in_range() {
    let (lo, hi) = (10u32, 100u32);
    for value in [0u32, 9, 10, 55, 100, 101, u32::MAX] {
        let got = clamp(value, lo, hi);
        assert!(
            got >= lo && got <= hi,
            "clamp({value}) = {got} out of range"
        );
    }
}

#[test]
fn midpoint_stays_between_inputs() {
    let cases = [
        (0u32, 0u32),
        (1, 2),
        (0, u32::MAX),
        (u32::MAX, u32::MAX),
        (7, 9),
    ];
    for (a, b) in cases {
        let mid = midpoint(a, b);
        assert!(
            mid >= a && mid <= b,
            "midpoint({a}, {b}) = {mid} out of range"
        );
    }
}

#[test]
fn midpoint_is_the_half_sum() {
    let cases = [(0u32, 0u32), (1, 2), (0, u32::MAX), (u32::MAX, u32::MAX)];
    for (a, b) in cases {
        let expected = a / 2 + b / 2 + (a % 2 + b % 2) / 2;
        assert_eq!(midpoint(a, b), expected, "midpoint({a}, {b})");
    }
}
