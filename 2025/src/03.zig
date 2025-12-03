const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/03.txt");
const pow10 = [_]usize{ 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000, 10000000000, 100000000000, 1000000000000, 10000000000000, 100000000000000, 1000000000000000, 10000000000000000, 100000000000000000, 1000000000000000000, 10000000000000000000 };

fn find_battery_joltage(comptime n: usize, line: []const u8) usize {
    var enabled_batteries = [_]usize{0} ** n;

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
        joltage += (pow10[enabled_batteries.len - i - 1] * value);
    }
    return joltage;
}

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, "\n");
    while (it.next()) |line| {
        if (line.len == 0) break;
        sum += find_battery_joltage(2, line);
    }

    return sum;
}

pub fn part2(input: []const u8, _: std.mem.Allocator) !usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, "\n");
    while (it.next()) |line| {
        if (line.len == 0) break;
        sum += find_battery_joltage(12, line);
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
