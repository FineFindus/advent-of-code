const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/02.txt");
const pow10 = [_]u128{ 1, 10, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, 1000000000, 10000000000, 100000000000, 1000000000000, 10000000000000, 100000000000000, 1000000000000000, 10000000000000000, 100000000000000000, 1000000000000000000, 10000000000000000000, 100000000000000000000, 1000000000000000000000, 10000000000000000000000 };

fn isSymmetric(id: usize) bool {
    var n = id;
    var digits: usize = 0;
    var pow: usize = 1;

    while (n != 0) : (n /= 10) {
        pow *= 10;
        digits += 1;
    }

    if (digits == 0 or digits % 2 != 0) return false;

    pow /= 10;

    const half = digits / 2;
    const divisor = pow10[half];

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
            if (isSymmetric(id)) {
                sum += id;
            }
        }
    }
    return sum;
}

fn isRepeating(id: usize) bool {
    const len = std.math.log10(id) + 1;
    // pattern can be at most len repeated digits
    pattern: for (1..(len / 2) + 1) |pattern_len| {
        // pattern length must by a divisor to evenly divide the ID
        if (@mod(len, pattern_len) != 0) continue;

        const sequence = id % pow10[pattern_len];
        // check if the next pattn_len digits are the same
        for (1..(len / pattern_len)) |k| {
            const next = (id / pow10[k * pattern_len]) % pow10[pattern_len];
            // not the same digits, try another pattern
            if (next != sequence) {
                continue :pattern;
            }
        }
        return true;
    }

    return false;
}

pub fn part2(input: []const u8, _: std.mem.Allocator) !usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, ",");
    while (it.next()) |raw_line| {
        if (raw_line.len == 0) break;
        const line = std.mem.trim(u8, raw_line, "\n");

        var id_iterator = std.mem.splitSequence(u8, line, "-");
        const lower = try std.fmt.parseInt(usize, id_iterator.next().?, 10);
        const upper = try std.fmt.parseInt(usize, id_iterator.next().?, 10);

        for (lower..(upper + 1)) |id| {
            if (isRepeating(id)) {
                sum += id;
            }
        }
    }
    return sum;
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
    try std.testing.expectEqual(4174379265, part2(example, gpa));
}
