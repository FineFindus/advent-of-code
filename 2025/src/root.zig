const std = @import("std");
const time = std.time;
const Timer = time.Timer;

pub fn solve(input: []const u8, part: usize, comptime func: fn (input: []const u8, alloc: std.mem.Allocator) anyerror!usize, alloc: std.mem.Allocator) !void {
    var timer = try Timer.start();
    const result = try func(input, alloc);
    const elapsed_us = @as(f64, @floatFromInt(timer.read())) / std.time.ns_per_us;

    std.debug.print("Part {d}: \x1b[1m{d}\x1b[0m ({d:.2}µs)\n", .{ part, result, elapsed_us });
}
