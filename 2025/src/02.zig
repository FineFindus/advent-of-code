const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/02.txt");

fn isInvalidId(id: usize) bool {
    var n = id;
    var digits: usize = 0;
    var pow10: usize = 1;

    while (n != 0) : (n /= 10) {
        pow10 *= 10;
        digits += 1;
    }

    if (digits == 0 or digits % 2 != 0) return false;

    pow10 /= 10;

    const half = digits / 2;
    const divisor = std.math.pow(usize, 10, half);

    const left = id / divisor;
    const right = id % divisor;

    return left == right;
}

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, ",");
    while (it.next()) |raw_line| {
        if (raw_line.len == 0) break;
        const line = std.mem.trim(u8, raw_line, "\n");

        var id_iterator = std.mem.splitSequence(u8, line, "-");
        const lower = try std.fmt.parseInt(usize, id_iterator.next().?, 10);
        const upper = try std.fmt.parseInt(usize, id_iterator.next().?, 10);

        for (lower..(upper + 1)) |id| {
            if (isInvalidId(id)) {
                sum += id;
            }
        }
    }
    return sum;
}

pub fn part2(_: []const u8, _: std.mem.Allocator) !usize {
    return 0;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/02.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(1227775554, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(0, part2(example, gpa));
}
