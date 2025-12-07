const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/07.txt");

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    const source_position = std.mem.indexOfPos(u8, input, 0, "S").?;
    // input has a size of 155 columns
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

pub fn part2(input: []const u8, _: std.mem.Allocator) !usize {
    const line_length = std.mem.indexOfPos(u8, input, 0, "\n").? + 1;
    const source_position = std.mem.indexOfPos(u8, input, 0, "S").?;
    // input has a size of 155 columns
    var rows = [_]usize{1} ** 155;

    var position = input.len;
    while (position > 0) : (position -= line_length) {
        const line = input[position - line_length .. position];

        for (line, 0..) |char, index| {
            if (char == '^') {
                rows[index] = rows[index - 1] + rows[index + 1];
            }
        }
    }
    return rows[source_position];
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
