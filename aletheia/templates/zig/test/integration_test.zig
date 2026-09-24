// SPDX-License-Identifier: MPL-2.0
//! Integration tests: exercise the public API as a consumer would.

const std = @import("std");
const core = @import("@@MOD_NAME@@");

test "clamp is idempotent" {
    const values = [_]u32{ 0, 1, 5, 42, std.math.maxInt(u32) };
    for (values) |value| {
        const once = core.clamp(value, 10, 100);
        try std.testing.expectEqual(once, core.clamp(once, 10, 100));
    }
}

test "clamp result is always inside the range" {
    const lo: u32 = 10;
    const hi: u32 = 100;
    const values = [_]u32{ 0, 9, 10, 55, 100, 101, std.math.maxInt(u32) };
    for (values) |value| {
        const got = core.clamp(value, lo, hi);
        try std.testing.expect(got >= lo and got <= hi);
    }
}

test "meanFloor never exceeds either input" {
    const cases = [_][2]u32{
        .{ 0, 0 },
        .{ 1, 2 },
        .{ std.math.maxInt(u32), 0 },
        .{ std.math.maxInt(u32), std.math.maxInt(u32) },
        .{ 7, 9 },
    };
    for (cases) |case| {
        const mean = core.meanFloor(case[0], case[1]);
        try std.testing.expect(mean <= @max(case[0], case[1]));
    }
}
