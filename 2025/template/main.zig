const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/day.txt");

pub fn part1(_: []const u8, _: std.mem.Allocator) !usize {
    return 0;
}

pub fn part2(_: []const u8, _: std.mem.Allocator) !usize {
    return 0;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/day.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(0, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(0, part2(example, gpa));
}
