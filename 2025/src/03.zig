const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/03.txt");

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, "\n");
    while (it.next()) |line| {
        if (line.len == 0) break;
        var first: usize = line[0] - '0';
        var second: usize = line[1] - '0';

        for (2..(line.len - 1)) |index| {
            const value = line[index] - '0';

            if (value > first) {
                first = value;
                second = line[index + 1] - '0';
            } else if (value > second) {
                second = value;
            }
        }
        // special case the last char, we cannot pick it as the first value
        const last = line[line.len - 1] - '0';
        if (last > second) {
            second = last;
        }

        sum += (first * 10) + second;
    }

    return sum;
}

pub fn part2(input: []const u8, _: std.mem.Allocator) !usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, "\n");
    while (it.next()) |line| {
        if (line.len == 0) break;
        var enabled_batteries = [_]usize{0} ** 12;
        var next_line_start: usize = 0;
        for (0..enabled_batteries.len) |i| {
            for (next_line_start..(line.len - (enabled_batteries.len - i - 1))) |line_index| {
                const digit = line[line_index] - '0';

                if (enabled_batteries[i] < digit) {
                    next_line_start = line_index + 1;
                    enabled_batteries[i] = digit;
                }
            }
        }

        var joltage: usize = 0;
        for (enabled_batteries, 0..) |value, i| {
            joltage += (std.math.pow(usize, 10, enabled_batteries.len - i - 1) * value);
        }
        sum += joltage;
    }

    return sum;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/03.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(357, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(3121910778619, part2(example, gpa));
}
