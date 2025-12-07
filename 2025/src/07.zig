const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/07.txt");

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    const source_position = std.mem.indexOfPos(u8, input, 0, "S").?;
    // input has a size of 155 lines
    var rows: u256 = std.math.shl(u256, 1, source_position);
    var splits: usize = 0;

    var it = std.mem.splitSequence(u8, input, "\n");
    // skip inital source line, we already handled it
    _ = it.next();
    while (it.next()) |line| {
        if (line.len == 0) break;

        for (line, 0..) |char, i| {
            if (char == '^') {
                const mask = std.math.shl(u256, 1, i);
                if (rows & mask != 0) {
                    splits += 1;
                }

                rows &= ~mask;
                rows |= (mask << 1);
                rows |= (mask >> 1);
            }
        }
    }
    return splits;
}

fn countTimelines(input: []const u8, source: usize, line_length: usize, cache: *std.AutoHashMap(usize, usize)) !usize {
    if (cache.get(source)) |count| {
        return count;
    }

    var position = source;
    // find next splitter
    while (true) {
        position += line_length + 1;

        if (position >= input.len) {
            // timeline reaches end
            try cache.put(source, 1);
            return 1;
        }

        if (input[position] == '^') break;
    }

    const left = try countTimelines(input, position - 1, line_length, cache);
    const right = try countTimelines(input, position + 1, line_length, cache);

    try cache.put(source, left + right);
    return left + right;
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    const line_length = std.mem.indexOfPos(u8, input, 0, "\n").?;

    const source_position = std.mem.indexOfPos(u8, input, 0, "S").?;
    var cache = std.AutoHashMap(usize, usize).init(allocator);
    defer cache.deinit();

    return try countTimelines(input, source_position, line_length, &cache);
}

pub fn main() !void {
    const input = @embedFile("data/inputs/07.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(21, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(40, part2(example, gpa));
}
