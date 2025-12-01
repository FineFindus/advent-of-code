const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/01.txt");
const data = @embedFile("data/inputs/01.txt");

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    var counter: usize = 0;
    var dial: isize = 50;
    var it = std.mem.splitSequence(u8, input, "\n");

    while (it.next()) |line| {
        if (line.len == 0) break;

        const rotations = try std.fmt.parseInt(i32, line[1..], 10);
        const direction = line[0];
        dial = switch (direction) {
            'L' => @mod(dial -% rotations, 100),
            'R' => @mod(dial + rotations, 100),
            else => dial,
        };
        if (dial == 0) counter += 1;
    }
    return counter;
}

pub fn part2(input: []const u8, _: std.mem.Allocator) !usize {
    var counter: usize = 0;
    var dial: isize = 50;
    var it = std.mem.splitSequence(u8, input, "\n");

    while (it.next()) |line| {
        if (line.len == 0) break;

        const direction = line[0];
        const rotations = try std.fmt.parseInt(isize, line[1..], 10);

        counter += @intCast(@divTrunc(rotations, 100));
        switch (direction) {
            'L' => {
                const new = dial - @mod(rotations, 100);
                if (new < 0 and dial != 0) {
                    counter += 1;
                }
                dial = @mod(new, 100);
            },
            'R' => {
                const new = dial + @mod(rotations, 100);
                if (new > 100 and dial != 100) {
                    counter += 1;
                }
                dial = @mod(new, 100);
            },
            else => {},
        }
        if (dial == 0) counter += 1;
    }
    return counter;
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
    try std.testing.expectEqual(6, part2(example, gpa));
}
