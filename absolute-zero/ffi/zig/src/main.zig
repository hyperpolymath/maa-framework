// Absolute Zero FFI Implementation
//
// This module implements the C-compatible FFI declared in src/abi/Foreign.idr.
// All types and layouts must match the Idris2 ABI definitions.
//
// The FFI provides a pure Zig implementation of the Brainfuck CNO verification
// engine, callable from any language that supports the C ABI.
//
// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>

const std = @import("std");

const VERSION = "1.0.0";
const BUILD_INFO = "absolute-zero built with Zig " ++ @import("builtin").zig_version_string;

/// Thread-local error storage
threadlocal var last_error: ?[]const u8 = null;

fn setError(msg: []const u8) void {
    last_error = msg;
}

fn clearError() void {
    last_error = null;
}

//==============================================================================
// Core Types (must match src/abi/Types.idr)
//==============================================================================

/// Result codes (matches Idris2 Result type: Ok=0..NotCNO=7)
pub const Result = enum(c_int) {
    ok = 0,
    err = 1,
    invalid_param = 2,
    out_of_memory = 3,
    null_pointer = 4,
    non_terminating = 5,
    has_side_effects = 6,
    not_cno = 7,
};

/// CNO verification result (matches Idris2 CNOVerificationResult)
/// Layout: 5 bools packed as u8, total 5 bytes
pub const CnoResult = extern struct {
    is_cno: u8,
    terminates: u8,
    preserves_state: u8,
    is_pure: u8,
    is_reversible: u8,
};

const MEMORY_SIZE: usize = 30_000;
const DEFAULT_MAX_CYCLES: usize = 1_000_000;

//==============================================================================
// Brainfuck VM (matches Rust BrainfuckInterpreter)
//==============================================================================

/// Brainfuck interpreter state
const BfInterpreter = struct {
    program: []const u8,
    bracket_map: []usize,
    memory: [MEMORY_SIZE]u8,
    pointer: usize,
    pc: usize,
    output_len: usize,
    halted: bool,
    cycles: usize,
    max_cycles: usize,
    allocator: std.mem.Allocator,

    fn init(allocator: std.mem.Allocator, program: []const u8, max_cycles: usize) !*BfInterpreter {
        const self = try allocator.create(BfInterpreter);
        self.* = .{
            .program = program,
            .bracket_map = try buildBracketMap(allocator, program),
            .memory = [_]u8{0} ** MEMORY_SIZE,
            .pointer = 0,
            .pc = 0,
            .output_len = 0,
            .halted = false,
            .cycles = 0,
            .max_cycles = max_cycles,
            .allocator = allocator,
        };
        return self;
    }

    fn deinit(self: *BfInterpreter) void {
        self.allocator.free(self.bracket_map);
        self.allocator.destroy(self);
    }

    fn step(self: *BfInterpreter) void {
        if (self.halted or self.pc >= self.program.len) {
            self.halted = true;
            return;
        }
        if (self.cycles >= self.max_cycles) {
            self.halted = true;
            return;
        }

        self.cycles += 1;
        const inst = self.program[self.pc];

        switch (inst) {
            '>' => self.pointer = (self.pointer + 1) % MEMORY_SIZE,
            '<' => self.pointer = if (self.pointer == 0) MEMORY_SIZE - 1 else self.pointer - 1,
            '+' => self.memory[self.pointer] +%= 1,
            '-' => self.memory[self.pointer] -%= 1,
            '.' => self.output_len += 1,
            ',' => self.memory[self.pointer] = 0, // no input in verification
            '[' => {
                if (self.memory[self.pointer] == 0) {
                    self.pc = self.bracket_map[self.pc];
                }
            },
            ']' => {
                if (self.memory[self.pointer] != 0) {
                    self.pc = self.bracket_map[self.pc];
                }
            },
            else => {},
        }
        self.pc += 1;
    }

    fn run(self: *BfInterpreter) void {
        while (!self.halted and self.pc < self.program.len) {
            self.step();
        }
        self.halted = true;
    }

    fn verifyCno(self: *BfInterpreter) CnoResult {
        var initial_memory: [MEMORY_SIZE]u8 = undefined;
        @memcpy(&initial_memory, &self.memory);
        const initial_pointer = self.pointer;

        self.run();

        const terminates = self.cycles < self.max_cycles;
        const is_pure = self.output_len == 0;
        const mem_preserved = std.mem.eql(u8, &self.memory, &initial_memory);
        const ptr_preserved = self.pointer == initial_pointer;
        const preserves_state = mem_preserved and ptr_preserved;
        const is_reversible = preserves_state;
        const is_cno = terminates and is_pure and preserves_state;

        return .{
            .is_cno = if (is_cno) 1 else 0,
            .terminates = if (terminates) 1 else 0,
            .preserves_state = if (preserves_state) 1 else 0,
            .is_pure = if (is_pure) 1 else 0,
            .is_reversible = if (is_reversible) 1 else 0,
        };
    }

    fn reset(self: *BfInterpreter) void {
        @memset(&self.memory, 0);
        self.pointer = 0;
        self.pc = 0;
        self.output_len = 0;
        self.halted = false;
        self.cycles = 0;
    }
};

fn buildBracketMap(allocator: std.mem.Allocator, program: []const u8) ![]usize {
    const map = try allocator.alloc(usize, program.len);
    @memset(map, 0);

    var stack_buf: [1024]usize = undefined;
    var stack_len: usize = 0;

    for (program, 0..) |ch, i| {
        if (ch == '[') {
            if (stack_len >= stack_buf.len) return error.NestingTooDeep;
            stack_buf[stack_len] = i;
            stack_len += 1;
        } else if (ch == ']') {
            if (stack_len > 0) {
                stack_len -= 1;
                const open = stack_buf[stack_len];
                map[open] = i;
                map[i] = open;
            }
        }
    }

    return map;
}

//==============================================================================
// C ABI Exports (matches src/abi/Foreign.idr declarations)
//==============================================================================

/// Initialize the library. Returns opaque handle.
export fn absolute_zero_init() ?*anyopaque {
    const allocator = std.heap.c_allocator;
    _ = allocator;
    clearError();
    // Library-level init (stateless for now)
    return @ptrFromInt(@as(usize, 1)); // Sentinel non-null
}

/// Free library resources.
export fn absolute_zero_free(_: ?*anyopaque) void {
    clearError();
}

/// Create a Brainfuck interpreter for CNO verification.
/// program_ptr: pointer to program bytes
/// program_len: length of program
/// max_cycles: cycle limit (0 = default 1M)
/// Returns opaque interpreter handle, or null on failure.
export fn az_bf_create(
    program_ptr: ?[*]const u8,
    program_len: u32,
    max_cycles: u32,
) ?*anyopaque {
    const prog = program_ptr orelse {
        setError("Null program pointer");
        return null;
    };

    const allocator = std.heap.c_allocator;
    const cycles: usize = if (max_cycles == 0) DEFAULT_MAX_CYCLES else @intCast(max_cycles);
    const program = prog[0..@intCast(program_len)];

    const interp = BfInterpreter.init(allocator, program, cycles) catch {
        setError("Failed to create interpreter");
        return null;
    };

    clearError();
    return @ptrCast(interp);
}

/// Destroy a Brainfuck interpreter.
export fn az_bf_destroy(handle: ?*anyopaque) void {
    const interp: *BfInterpreter = @ptrCast(@alignCast(handle orelse return));
    interp.deinit();
    clearError();
}

/// Run CNO verification on a Brainfuck program.
/// Returns CnoResult struct (matches Idris2 CNOVerificationResult).
export fn az_bf_verify_cno(handle: ?*anyopaque) CnoResult {
    const interp: *BfInterpreter = @ptrCast(@alignCast(handle orelse {
        setError("Null interpreter handle");
        return .{ .is_cno = 0, .terminates = 0, .preserves_state = 0, .is_pure = 0, .is_reversible = 0 };
    }));

    clearError();
    return interp.verifyCno();
}

/// Reset interpreter for re-verification.
export fn az_bf_reset(handle: ?*anyopaque) Result {
    const interp: *BfInterpreter = @ptrCast(@alignCast(handle orelse {
        setError("Null handle");
        return .null_pointer;
    }));
    interp.reset();
    clearError();
    return .ok;
}

//==============================================================================
// Error & Version
//==============================================================================

/// Get the last error message. Returns null if no error.
export fn absolute_zero_last_error() ?[*:0]const u8 {
    const err = last_error orelse return null;
    const allocator = std.heap.c_allocator;
    const c_str = allocator.dupeZ(u8, err) catch return null;
    return c_str.ptr;
}

/// Get the library version string.
export fn absolute_zero_version() [*:0]const u8 {
    return VERSION;
}

/// Get build information.
export fn absolute_zero_build_info() [*:0]const u8 {
    return BUILD_INFO;
}

//==============================================================================
// Tests
//==============================================================================

test "empty program is CNO" {
    const allocator = std.testing.allocator;
    const interp = try BfInterpreter.init(allocator, "", DEFAULT_MAX_CYCLES);
    defer interp.deinit();

    const result = interp.verifyCno();
    try std.testing.expectEqual(@as(u8, 1), result.is_cno);
    try std.testing.expectEqual(@as(u8, 1), result.terminates);
    try std.testing.expectEqual(@as(u8, 1), result.is_pure);
    try std.testing.expectEqual(@as(u8, 1), result.preserves_state);
}

test "balanced move is CNO" {
    const allocator = std.testing.allocator;
    const interp = try BfInterpreter.init(allocator, "><", DEFAULT_MAX_CYCLES);
    defer interp.deinit();

    const result = interp.verifyCno();
    try std.testing.expectEqual(@as(u8, 1), result.is_cno);
}

test "balanced inc/dec is CNO" {
    const allocator = std.testing.allocator;
    const interp = try BfInterpreter.init(allocator, "+-", DEFAULT_MAX_CYCLES);
    defer interp.deinit();

    const result = interp.verifyCno();
    try std.testing.expectEqual(@as(u8, 1), result.is_cno);
}

test "unbalanced increment is NOT CNO" {
    const allocator = std.testing.allocator;
    const interp = try BfInterpreter.init(allocator, "+", DEFAULT_MAX_CYCLES);
    defer interp.deinit();

    const result = interp.verifyCno();
    try std.testing.expectEqual(@as(u8, 0), result.is_cno);
    try std.testing.expectEqual(@as(u8, 0), result.preserves_state);
}

test "output is NOT CNO" {
    const allocator = std.testing.allocator;
    const interp = try BfInterpreter.init(allocator, ".", DEFAULT_MAX_CYCLES);
    defer interp.deinit();

    const result = interp.verifyCno();
    try std.testing.expectEqual(@as(u8, 0), result.is_cno);
    try std.testing.expectEqual(@as(u8, 0), result.is_pure);
}

test "CnoResult layout matches Idris2" {
    // CNOVerificationResult in Types.idr has size 5
    try std.testing.expectEqual(@as(usize, 5), @sizeOf(CnoResult));
}
