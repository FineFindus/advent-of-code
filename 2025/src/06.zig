const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/06.txt");

pub fn part1(input: []const u8, _: std.mem.Allocator) !usize {
    const line_length = std.mem.indexOfPos(u8, input, 0, "\n").?;
    // all lines have the same length
    const lines = input.len / line_length;

    var sum: usize = 0;
    var column: usize = 0;
    while (column < line_length) {
        const symbol = input[(lines - 1) * (line_length + 1) + column];
        var intermediate_result: usize = if (symbol == '*') 1 else 0;

        var num_len: usize = 0;
        for (0..lines - 1) |row| {
            const line = input[row * (line_length + 1) + column ..];

            var position: usize = 0;
            while (line[position] == ' ') : (position += 1) {}

            var next_empty = position + 1;
            while (next_empty < line.len and (line[next_empty] != ' ' and line[next_empty] != '\n')) : (next_empty += 1) {}
            num_len = @max(num_len, next_empty);

            const value = try std.fmt.parseInt(usize, line[position..next_empty], 10);

            if (symbol == '*') {
                intermediate_result *= value;
            } else {
                intermediate_result += value;
            }
        }
        column += num_len + 1;
        sum += intermediate_result;
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
