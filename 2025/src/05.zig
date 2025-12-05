const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/05.txt");

const Interval = struct {
    lower: usize,
    upper: usize,

    fn sortFn(context: void, lhs: Interval, rhs: Interval) bool {
        _ = context;
        return lhs.lower < rhs.lower;
    }
};

pub fn part1(input: []const u8, allocator: std.mem.Allocator) !usize {
    var input_it = std.mem.splitSequence(u8, input, "\n\n");
    var fresh_ingredients_it = std.mem.splitSequence(u8, input_it.next().?, "\n");

    var fresh_ingredients = try std.ArrayList(Interval).initCapacity(allocator, 1024);
    defer fresh_ingredients.deinit(allocator);

    while (fresh_ingredients_it.next()) |line| {
        if (line.len == 0) break;

        var it = std.mem.splitSequence(u8, line, "-");
        const lower = try std.fmt.parseInt(usize, it.next().?, 10);
        const upper = try std.fmt.parseInt(usize, it.next().?, 10);
        try fresh_ingredients.append(allocator, Interval{ .lower = lower, .upper = upper });
    }

    var sum: usize = 0;
    var available_ingredients = std.mem.splitSequence(u8, input_it.next().?, "\n");
    while (available_ingredients.next()) |line| {
        if (line.len == 0) break;
        const id = try std.fmt.parseInt(usize, line, 10);

        for (fresh_ingredients.items) |value| {
            if (value.lower <= id and id <= value.upper) {
                sum += 1;
                break;
            }
        }
    }

    return sum;
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    var input_it = std.mem.splitSequence(u8, input, "\n\n");
    var fresh_ingredients_it = std.mem.splitSequence(u8, input_it.next().?, "\n");

    var fresh_ingredients = try std.ArrayList(Interval).initCapacity(allocator, 256);
    defer fresh_ingredients.deinit(allocator);

    while (fresh_ingredients_it.next()) |line| {
        if (line.len == 0) break;

        var it = std.mem.splitSequence(u8, line, "-");
        const lower = try std.fmt.parseInt(usize, it.next().?, 10);
        const upper = try std.fmt.parseInt(usize, it.next().?, 10);

        try fresh_ingredients.append(allocator, Interval{ .lower = lower, .upper = upper });
    }

    std.sort.pdq(Interval, fresh_ingredients.items, {}, Interval.sortFn);

    var bound: usize = 0;
    var sum: usize = 0;
    for (fresh_ingredients.items) |value| {
        var lower = value.lower;
        const upper = value.upper + 1;

        if (lower < bound) {
            lower = bound;
        }
        if (upper <= bound) continue;

        sum += upper - lower;
        bound = upper;
    }

    return sum;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/05.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(3, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(14, part2(example, gpa));
}
