const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/04.txt");

const Grid = struct {
    width: usize,
    height: usize,
    buffer: []u8,

    fn from_buffer(buffer: []u8) Grid {
        var it = std.mem.splitSequence(u8, buffer, "\n");
        const width = it.peek().?.len + 1;
        var height: usize = 0;
        while (it.next()) |line| {
            if (line.len != 0) height += 1;
        }

        return .{
            .width = width,
            .height = height,
            .buffer = buffer,
        };
    }

    fn at(self: Grid, x: isize, y: isize) ?u8 {
        if (x < 0 or x >= self.width) {
            return null;
        }

        if (y < 0 or y >= self.height) {
            return null;
        }

        const positive_x: usize = @intCast(x);
        const positive_y: usize = @intCast(y);
        return self.at_unchecked(positive_x, positive_y);
    }

    fn set(self: Grid, x: isize, y: isize, value: u8) void {
        if (x < 0 or x >= self.width) {
            return;
        }

        if (y < 0 or y >= self.height) {
            return;
        }

        const positive_x: usize = @intCast(x);
        const positive_y: usize = @intCast(y);
        self.buffer[positive_y * self.width + positive_x] = value;
    }

    fn at_unchecked(self: Grid, x: usize, y: usize) ?u8 {
        return self.buffer[y * self.width + x];
    }

    fn debug_print(self: Grid) void {
        for (0..self.height) |row| {
            for (0..self.width) |column| {
                std.debug.print("{c}", .{self.at_unchecked(column, row).?});
            }
        }
    }
};

fn count(input: []const u8, grid: *Grid, comptime remove: bool) usize {
    var sum: usize = 0;

    var it = std.mem.splitSequence(u8, input, "\n");
    var y: isize = 0;
    while (it.next()) |line| : (y += 1) {
        if (line.len == 0) break;

        var x: isize = 0;
        while (x < line.len) : (x += 1) {
            const current_cell = grid.at(x, y).?;
            if (current_cell != '@') continue;

            var neighbours: usize = 0;

            for (0..3) |i| {
                for (0..3) |j| {
                    if (i == 1 and j == 1) continue;

                    const isize_i: isize = @intCast(i);
                    const isize_j: isize = @intCast(j);
                    const neighbour_cell = grid.at(x + isize_i - 1, y + isize_j - 1);
                    if (neighbour_cell == '@') {
                        neighbours += 1;
                    }
                }
            }

            if (neighbours < 4) {
                sum += 1;
                if (remove) grid.set(x, y, '.');
            }
        }
    }
    return sum;
}

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    //SAFETY: we never mutate the array/call `set` on the grid
    const buffer = @constCast(input);
    var grid = Grid.from_buffer(buffer);

    return count(input, &grid, false);
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    const buffer = try allocator.alloc(u8, input.len);
    @memcpy(buffer, input);
    defer allocator.free(buffer);
    var grid = Grid.from_buffer(buffer);

    var sum: usize = 0;
    while (true) {
        const removed = count(input, &grid, true);
        if (removed == 0) break;
        sum += removed;
    }

    return sum;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/04.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(13, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(43, part2(example, gpa));
}
