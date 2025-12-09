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
    for (points.items, 0..) |a, i| {
        for (i + 1..points.items.len) |j| {
            const b = points.items[j];
            const area = (a.x - b.x + 1) * (a.y - b.y + 1);
            max = @max(max, @abs(area));
        }
    }

    return max;
}

fn liang_barsky(top_left: Point, bottom_right: Point, line_start: Point, line_end: Point) bool {
    const dx: f32 = @floatFromInt(line_end.x - line_start.x);
    const dy: f32 = @floatFromInt(line_end.y - line_start.y);
    const p = [_]f32{ -dx, dx, -dy, dy };
    const q = [_]f32{
        @floatFromInt(line_start.x - top_left.x),
        @floatFromInt(bottom_right.x - line_start.x),
        @floatFromInt(line_start.y - top_left.y),
        @floatFromInt(bottom_right.y - line_start.y),
    };

    var t_enter: f32 = 0;
    var t_exit: f32 = 1;

    for (0..4) |i| {
        // check if line is parallel to the clipping boundary
        if (p[i] == 0) {
            if (q[i] < 0) {
                return false;
            }
            continue;
        }
        const t = q[i] / p[i];
        // const t = @divTrunc(q[i], p[i]);
        if (p[i] < 0) {
            t_enter = @max(t_enter, t);
        } else {
            t_exit = @min(t_exit, t);
        }
    }

    if (t_enter > t_exit) {
        // line is completely outside
        return false;
    }

    return true;
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    var points = try Point.fromStr(input, allocator);
    defer points.deinit(allocator);

    var max: usize = 0;
    for (points.items, 0..) |a, i| {
        points: for (i + 1..points.items.len) |j| {
            const b = points.items[j];

            // use a line intersection algorithm to check if all edges lie outside the rect
            const top_left = Point{ .x = @min(a.x, b.x), .y = @min(a.y, b.y) };
            const bottom_right = Point{ .x = @max(a.x, b.x), .y = @max(a.y, b.y) };

            const area = (@abs(top_left.x - bottom_right.x) + 1) * (@abs(top_left.y - bottom_right.y) + 1);
            if (area < max) continue;

            for (0..points.items.len) |index| {
                const line_start = points.items[index];
                const line_end = points.items[(index + 1) % points.items.len];

                if (liang_barsky(top_left, bottom_right, line_start, line_end)) {
                    const line_min_x = @min(line_start.x, line_end.x);
                    const line_max_x = @max(line_start.x, line_end.x);
                    const line_min_y = @min(line_start.y, line_end.y);
                    const line_max_y = @max(line_start.y, line_end.y);

                    // check if the line is fully interior
                    if (top_left.x < line_max_x and bottom_right.x > line_min_x and
                        top_left.y < line_max_y and bottom_right.y > line_min_y)
                    {
                        continue :points;
                    }
                }
            }

            max = @max(max, @abs(area));
        }
    }

    return max;
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
    try std.testing.expectEqual(24, part2(example, gpa));
}
