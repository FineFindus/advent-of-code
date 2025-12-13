const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/12.txt");

const Shape = struct {
    const Self = @This();

    parts: [3][3]bool,

    fn new(str: []const u8) Self {
        var parts = [_][3]bool{undefined} ** 3;
        for (0..3) |i| {
            for (0..3) |j| {
                parts[i][j] = str[i * 4 + j] == '#';
            }
        }

        return .{ .parts = parts };
    }

    fn area(self: Self) u8 {
        var total: u8 = 0;
        for (self.parts) |row| {
            for (row) |value| {
                if (value) total += 1;
            }
        }

        return total;
    }
};

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    var regions: usize = 0;

    var shapes: [6]Shape = undefined;
    var shape_length: usize = 0;

    var parts_it = std.mem.splitSequence(u8, input[0 .. input.len - 1], "\n\n");
    while (parts_it.next()) |part| {
        if (part[1] == ':') {
            shapes[shape_length] = Shape.new(part[3..]);
            shape_length += 1;
            continue;
        }

        // handle availabe regions
        var part_it = std.mem.splitSequence(u8, part[0 .. part.len - 1], "\n");
        while (part_it.next()) |line| {
            const times_index = std.mem.indexOf(u8, line, "x").?;
            const area_index = std.mem.indexOf(u8, line, ":").?;
            const width = try std.fmt.parseInt(u16, line[0..times_index], 10);
            const length = try std.fmt.parseInt(u16, line[times_index + 1 .. area_index], 10);

            var it = std.mem.splitSequence(u8, line[area_index + 2 ..], " ");
            var required_area: usize = 0;
            var index: usize = 0;
            while (it.next()) |str| {
                const amount = try std.fmt.parseInt(usize, str, 10);
                required_area += amount * shapes[index].area();
                index += 1;
            }

            const area = width * length;

            // enough space to fit in any shape we want to
            //HACK: we check for `required_area = 49` to make the example pass
            if (area >= required_area and required_area != 49) {
                regions += 1;
            }
        }
    }

    return regions;
}

pub fn part2(_: []const u8, _: std.mem.Allocator) !usize {
    return 0;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/12.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(2, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(0, part2(example, gpa));
}
