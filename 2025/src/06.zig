const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/06.txt");

pub fn part1(input: []const u8, allocator: std.mem.Allocator) !usize {
    var operands = try std.ArrayList(usize).initCapacity(allocator, 4888);
    defer operands.deinit(allocator);

    var index: usize = 0;
    var it = std.mem.splitSequence(u8, input, "\n");
    var line_length: ?usize = null;
    while (it.next()) |line| {
        var position: usize = 0;
        while (position < line.len - 2) {
            while (line[position] == ' ') : (position += 1) {}

            var next_empty = position + 1;
            while (next_empty < line.len and line[next_empty] != ' ') : (next_empty += 1) {}

            const value = try std.fmt.parseInt(usize, line[position..next_empty], 10);
            try operands.append(allocator, value);

            position = next_empty;
        }

        if (line_length == null) line_length = operands.items.len;
        index += 1;

        const peek = it.peek().?;
        // next line is the operands
        if (peek[0] == '*' or peek[0] == '+') break;
    }

    var sum: usize = 0;

    const line = it.next().?;
    var position: usize = 0;
    var column: usize = 0;
    outer: while (position < line.len) : (position += 1) {
        while (line[position] == ' ') {
            if (position == line.len - 1) break :outer;
            position += 1;
        }

        var result: usize = if (line[position] == '*') 1 else 0;
        for (0..index) |i| {
            const value = operands.items[i * line_length.? + column];
            switch (line[position]) {
                '*' => result *= value,
                '+' => result += value,
                else => {},
            }
        }
        column += 1;
        sum += result;
    }

    return sum;
}

pub fn part2(input: []const u8, _: std.mem.Allocator) !usize {
    const line_length = std.mem.indexOfPos(u8, input, 0, "\n").?;
    // all lines have the same length
    const lines = input.len / line_length;

    var sum: usize = 0;
    var column: usize = 0;
    while (column < line_length) : (column += 1) {
        const symbol = input[(lines - 1) * (line_length + 1) + column];
        var intermediate_result: usize = if (symbol == '*') 1 else 0;

        while (column < line_length) : (column += 1) {
            var operand: usize = 0;

            var valid_lines: usize = 0;
            for (0..lines - 1) |row| {
                const digit = input[row * (line_length + 1) + column];
                if (digit == ' ') {
                    operand /= 10;
                    continue;
                }
                operand += (digit - '0') * std.math.pow(usize, 10, lines - 2 - row);
                valid_lines += 1;
            }
            if (operand == 0) break;

            if (symbol == '*') {
                intermediate_result *= operand;
            } else {
                intermediate_result += operand;
            }
            operand = 0;
        }

        sum += intermediate_result;
    }

    return sum;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/06.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(4277556, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(3263827, part2(example, gpa));
}
