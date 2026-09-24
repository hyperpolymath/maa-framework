// SPDX-License-Identifier: MPL-2.0
//! @@PROJECT_NAME@@ — command-line entry point.
//!
//! Written against the Zig 0.16 I/O API (`std.process.Init` + `std.Io`).

const std = @import("std");
const Io = std.Io;
const core = @import("@@MOD_NAME@@");

const usage =
    \\@@PROJECT_NAME@@ @@VERSION@@
    \\
    \\usage:
    \\  @@PROJECT_NAME@@ clamp <value> <lo> <hi>   clamp a value into a range
    \\  @@PROJECT_NAME@@ mean  <a> <b>             integer mean, rounded down
    \\  @@PROJECT_NAME@@ --help                    show this message
    \\
    \\All arguments are unsigned 32-bit integers.
;

pub fn main(init: std.process.Init) !void {
    const arena: std.mem.Allocator = init.arena.allocator();
    const io = init.io;
    const args = try init.minimal.args.toSlice(arena);

    var stdout_buffer: [1024]u8 = undefined;
    var stdout_file_writer: Io.File.Writer = .init(.stdout(), io, &stdout_buffer);
    const stdout = &stdout_file_writer.interface;

    var stderr_buffer: [1024]u8 = undefined;
    var stderr_file_writer: Io.File.Writer = .init(.stderr(), io, &stderr_buffer);
    const stderr = &stderr_file_writer.interface;

    if (args.len < 2) {
        try stdout.print("{s}\n", .{usage});
        try stdout.flush();
        return;
    }

    const cmd = args[1];
    if (std.mem.eql(u8, cmd, "--help") or std.mem.eql(u8, cmd, "-h")) {
        try stdout.print("{s}\n", .{usage});
        try stdout.flush();
        return;
    }

    if (std.mem.eql(u8, cmd, "clamp")) {
        if (args.len == 5) {
            const value = std.fmt.parseInt(u32, args[2], 10) catch null;
            const lo = std.fmt.parseInt(u32, args[3], 10) catch null;
            const hi = std.fmt.parseInt(u32, args[4], 10) catch null;
            if (value != null and lo != null and hi != null and lo.? <= hi.?) {
                try stdout.print("{d}\n", .{core.clamp(value.?, lo.?, hi.?)});
                try stdout.flush();
                return;
            }
        }
    } else if (std.mem.eql(u8, cmd, "mean")) {
        if (args.len == 4) {
            const a = std.fmt.parseInt(u32, args[2], 10) catch null;
            const b = std.fmt.parseInt(u32, args[3], 10) catch null;
            if (a != null and b != null) {
                try stdout.print("{d}\n", .{core.meanFloor(a.?, b.?)});
                try stdout.flush();
                return;
            }
        }
    }

    try stderr.print("error: unrecognised arguments\n\n{s}\n", .{usage});
    try stderr.flush();
    std.process.exit(1);
}
