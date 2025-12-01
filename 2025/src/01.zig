const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/01.txt");
const data = @embedFile("data/inputs/01.txt");

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    var counter: usize = 0;
    var dial: isize = 50;
    var it = std.mem.splitScalar(u8, input, '\n');
    while (it.next()) |val| {
        if (val.len == 0) {
            break;
        }
        const rotations: isize = @intCast(try std.fmt.parseInt(i32, val[1..], 10));
        switch (val[0]) {
            @as(u8, 'L') => {
                dial = @mod(dial -% rotations, 100);
            },
            @as(u8, 'R') => {
                dial = @mod(dial +% rotations, 100);
            },
            else => {},
        }
        if (dial == 0) {
            counter += 1;
        }
    }
    return counter;
}

pub fn part2(_: []const u8, _: std.mem.Allocator) !usize {
    return 0;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(data, 1, part1, allocator);
    try aoc.solve(data, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(3, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(0, part2(example, gpa));
}
