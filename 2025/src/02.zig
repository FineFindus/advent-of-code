const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/02.txt");

pub fn part1(input: []const u8, alloc: std.mem.Allocator) !usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, ",");
    while (it.next()) |raw_line| {
        if (raw_line.len == 0) break;
        const line = std.mem.trim(u8, raw_line, "\n");

        var id_iterator = std.mem.splitSequence(u8, line, "-");
        const lower = try std.fmt.parseInt(usize, id_iterator.next().?, 10);
        const upper = try std.fmt.parseInt(usize, id_iterator.next().?, 10);

        for (lower..(upper + 1)) |id| {
            const string = try std.fmt.allocPrint(
                alloc,
                "{d}",
                .{id},
            );
            defer alloc.free(string);
            if (@mod(string.len, 2) != 0) {
                continue;
            }

            const middle = string.len / 2;
            if (std.mem.eql(u8, string[0..middle], string[middle..])) {
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
