// Absolute Zero FFI Integration Tests
// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
//
// These tests verify that the Zig FFI correctly implements the Idris2 ABI.
// They call through the C ABI exports and verify CNO verification results.

const std = @import("std");
const testing = std.testing;

// Import FFI functions via C ABI
extern fn absolute_zero_init() ?*anyopaque;
extern fn absolute_zero_free(?*anyopaque) void;
extern fn absolute_zero_version() [*:0]const u8;
extern fn absolute_zero_last_error() ?[*:0]const u8;

const CnoResult = extern struct {
    is_cno: u8,
    terminates: u8,
    preserves_state: u8,
    is_pure: u8,
    is_reversible: u8,
};

extern fn az_bf_create(?[*]const u8, u32, u32) ?*anyopaque;
extern fn az_bf_destroy(?*anyopaque) void;
extern fn az_bf_verify_cno(?*anyopaque) CnoResult;
extern fn az_bf_reset(?*anyopaque) c_int;

//==============================================================================
// Lifecycle Tests
//==============================================================================

test "library init and free" {
    const handle = absolute_zero_init();
    try testing.expect(handle != null);
    absolute_zero_free(handle);
}

test "version string" {
    const ver = absolute_zero_version();
    const ver_str = std.mem.span(ver);
    try testing.expectEqualStrings("1.0.0", ver_str);
}

//==============================================================================
// Brainfuck CNO Verification via C ABI
//==============================================================================

test "empty program is CNO via FFI" {
    const program = "";
    const interp = az_bf_create(program.ptr, program.len, 0);
    try testing.expect(interp != null);
    defer az_bf_destroy(interp);

    const result = az_bf_verify_cno(interp);
    try testing.expectEqual(@as(u8, 1), result.is_cno);
    try testing.expectEqual(@as(u8, 1), result.terminates);
    try testing.expectEqual(@as(u8, 1), result.is_pure);
    try testing.expectEqual(@as(u8, 1), result.preserves_state);
}

test "balanced move is CNO via FFI" {
    const program = "><";
    const interp = az_bf_create(program.ptr, program.len, 0);
    defer az_bf_destroy(interp);

    const result = az_bf_verify_cno(interp);
    try testing.expectEqual(@as(u8, 1), result.is_cno);
}

test "unbalanced increment is NOT CNO via FFI" {
    const program = "+";
    const interp = az_bf_create(program.ptr, program.len, 0);
    defer az_bf_destroy(interp);

    const result = az_bf_verify_cno(interp);
    try testing.expectEqual(@as(u8, 0), result.is_cno);
    try testing.expectEqual(@as(u8, 0), result.preserves_state);
}

test "output is NOT pure via FFI" {
    const program = ".";
    const interp = az_bf_create(program.ptr, program.len, 0);
    defer az_bf_destroy(interp);

    const result = az_bf_verify_cno(interp);
    try testing.expectEqual(@as(u8, 0), result.is_cno);
    try testing.expectEqual(@as(u8, 0), result.is_pure);
}

test "reset allows re-verification" {
    const program = "+-";
    const interp = az_bf_create(program.ptr, program.len, 0);
    defer az_bf_destroy(interp);

    const r1 = az_bf_verify_cno(interp);
    try testing.expectEqual(@as(u8, 1), r1.is_cno);

    const reset_result = az_bf_reset(interp);
    try testing.expectEqual(@as(c_int, 0), reset_result); // ok = 0

    const r2 = az_bf_verify_cno(interp);
    try testing.expectEqual(@as(u8, 1), r2.is_cno);
}

test "null handle returns zero result" {
    const result = az_bf_verify_cno(null);
    try testing.expectEqual(@as(u8, 0), result.is_cno);
}

//==============================================================================
// CnoResult ABI Layout
//==============================================================================

test "CnoResult size matches Idris2 CNOVerificationResult" {
    try testing.expectEqual(@as(usize, 5), @sizeOf(CnoResult));
}
