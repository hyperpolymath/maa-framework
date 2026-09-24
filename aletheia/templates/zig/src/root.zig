// SPDX-License-Identifier: MPL-2.0
//! Core library for @@PROJECT_NAME@@.
//!
//! Replace these sample functions with your own.

const std = @import("std");

/// Clamp `value` into the inclusive range `[lo, hi]`.
///
/// Asserts `lo <= hi`; an inverted range is a programming error, not a
/// silent no-op.
pub fn clamp(value: u32, lo: u32, hi: u32) u32 {
    std.debug.assert(lo <= hi);
    return @min(@max(value, lo), hi);
}

/// Integer mean of two `u32`s, rounded towards zero.
///
/// The naive `(a + b) / 2` overflows for large inputs; this form cannot,
/// because it never materialises the sum.
pub fn meanFloor(a: u32, b: u32) u32 {
    return (a & b) + ((a ^ b) >> 1);
}

test "clamp keeps value within range" {
    try std.testing.expectEqual(@as(u32, 5), clamp(5, 0, 10));
    try std.testing.expectEqual(@as(u32, 1), clamp(0, 1, 10));
    try std.testing.expectEqual(@as(u32, 10), clamp(99, 0, 10));
}

test "clamp handles a degenerate range" {
    try std.testing.expectEqual(@as(u32, 7), clamp(7, 7, 7));
}

test "meanFloor matches the naive form on small inputs" {
    var a: u32 = 0;
    while (a < 64) : (a += 1) {
        var b: u32 = 0;
        while (b < 64) : (b += 1) {
            try std.testing.expectEqual((a + b) / 2, meanFloor(a, b));
        }
    }
}

test "meanFloor does not overflow" {
    try std.testing.expectEqual(std.math.maxInt(u32), meanFloor(std.math.maxInt(u32), std.math.maxInt(u32)));
    try std.testing.expectEqual(std.math.maxInt(u32) / 2, meanFloor(std.math.maxInt(u32), 0));
}
