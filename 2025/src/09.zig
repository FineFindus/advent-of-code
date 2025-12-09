const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/09.txt");

const Point = struct {
    const Self = @This();

    x: isize,
    y: isize,

    /// Computes the euclidian distance between `self` and another point.
    fn distance(self: Self, rhs: Self) isize {
        return (self.x - rhs.x) * (self.x - rhs.x) + (self.y - rhs.y) * (self.y - rhs.y);
    }

    fn fromStr(input: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Self) {
        var points = try std.ArrayList(Point).initCapacity(allocator, 1000);

        var it = std.mem.splitSequence(u8, input, "\n");
        var length: usize = 0;
        while (it.next()) |line| : (length += 1) {
            if (line.len == 0) break;
            var coord_it = std.mem.splitSequence(u8, line, ",");
            const point = Self{
                .x = try std.fmt.parseInt(isize, coord_it.next().?, 10),
                .y = try std.fmt.parseInt(isize, coord_it.next().?, 10),
            };
            try points.append(allocator, point);
        }

        return points;
    }
};
pub fn part1(input: []const u8, allocator: std.mem.Allocator) !usize {
    var points = try Point.fromStr(input, allocator);
    defer points.deinit(allocator);

    var max: usize = 0;
    for (points.items) |a| {
        for (points.items) |b| {
            const area = (@abs(a.x - b.x) + 1) * (@abs(a.y - b.y) + 1);
            max = @max(max, area);
        }
    }

    return max;
}

pub fn part2(_: []const u8, _: std.mem.Allocator) !usize {
    return 0;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/09.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(50, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(0, part2(example, gpa));
}
