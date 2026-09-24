// SPDX-License-Identifier: MPL-2.0
// Implementation shared verbatim by the main crate and the Creusot crate.
//
// This file is the *single source of truth*. `src/lib.rs` and
// `verification/src/lib.rs` both `include!` it, so the specifications can
// never drift from the code they describe.
//
// The contract attributes are gated behind `cfg(creusot)`, which only
// `creusot-rustc` sets. Under a normal `cargo build` every `#[cfg_attr]`
// below vanishes and the Creusot prelude is never imported, so the main
// crate stays zero-dependency and builds air-gapped.

#[cfg(creusot)]
use creusot_std::prelude::*;

/// Clamp `value` into the inclusive range `[lo, hi]`.
///
/// # Panics
///
/// Panics if `lo > hi`, which would make the range incoherent.
///
/// Creusot proves that, given `lo <= hi`, the result always lies within
/// `[lo, hi]`.
#[cfg_attr(creusot, requires(lo@ <= hi@))]
#[cfg_attr(creusot, ensures(lo@ <= result@ && result@ <= hi@))]
#[must_use]
pub fn clamp(value: u32, lo: u32, hi: u32) -> u32 {
    assert!(lo <= hi, "incoherent range");
    value.max(lo).min(hi)
}

/// Overflow-free midpoint of `a` and `b`, rounded towards zero.
///
/// The naive `(a + b) / 2` overflows when both inputs are large; this form
/// cannot, because it never materialises the sum. Creusot proves the result
/// lies in `[a, b]` and equals the exact half-sum `(a + b) / 2`.
///
/// # Panics
///
/// Panics if `a > b` (the subtraction would underflow).
#[cfg_attr(creusot, requires(a@ <= b@))]
#[cfg_attr(creusot, ensures(a@ <= result@ && result@ <= b@))]
#[cfg_attr(creusot, ensures(result@ == (a@ + b@) / 2))]
#[must_use]
pub fn midpoint(a: u32, b: u32) -> u32 {
    a + (b - a) / 2
}
