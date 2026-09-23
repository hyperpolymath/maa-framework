// SPDX-License-Identifier: MPL-2.0
//! Core library for @@PROJECT_NAME@@.
//!
//! Replace these sample functions with your own. They are deliberately small
//! but carry *real* preconditions: Creusot discharges the matching proof
//! obligations, and those obligations are written against this very file —
//! `src/lib.rs` includes `impl.rs`, and so does `verification/src/lib.rs`.
//!
//! To change behaviour, edit `src/impl.rs`: both builds pick the change up,
//! so the proof can never drift from the implementation.

include!("impl.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_keeps_value_within_range() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(0, 1, 10), 1);
        assert_eq!(clamp(99, 0, 10), 10);
    }

    #[test]
    fn clamp_handles_degenerate_range() {
        assert_eq!(clamp(7, 7, 7), 7);
    }

    #[test]
    #[should_panic(expected = "incoherent range")]
    fn clamp_rejects_inverted_range() {
        let _ = clamp(5, 10, 0);
    }

    #[test]
    fn midpoint_matches_naive_on_small_inputs() {
        for a in 0..64u32 {
            for b in a..64u32 {
                assert_eq!(midpoint(a, b), (a + b) / 2, "a={a} b={b}");
            }
        }
    }

    #[test]
    fn midpoint_does_not_overflow() {
        assert_eq!(midpoint(u32::MAX, u32::MAX), u32::MAX);
        assert_eq!(midpoint(0, u32::MAX), u32::MAX / 2);
    }

    #[test]
    #[should_panic]
    fn midpoint_rejects_inverted_inputs() {
        let _ = midpoint(10, 0);
    }
}
