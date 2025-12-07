const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/07.txt");

// from https://ziglang.org/learn/samples/
pub fn Queue(comptime Child: type) type {
    return struct {
        const Node = struct {
            data: Child,
            next: ?*Node,
        };
        gpa: std.mem.Allocator,
        start: ?*Node,
        end: ?*Node,

        pub fn init(gpa: std.mem.Allocator) @This() {
            return @This(){
                .gpa = gpa,
                .start = null,
                .end = null,
            };
        }

        pub fn enqueue(self: *@This(), value: Child) !void {
            const node = try self.gpa.create(Node);
            node.* = .{ .data = value, .next = null };
            if (self.end) |end| end.next = node //
            else self.start = node;
            self.end = node;
        }

        pub fn dequeue(self: *@This()) ?Child {
            const start = self.start orelse return null;
            defer self.gpa.destroy(start);

            if (start.next) |next|
                self.start = next
            else {
                self.start = null;
                self.end = null;
            }
            return start.data;
        }
    };
}

pub fn part1(input: []const u8, allocator: std.mem.Allocator) !usize {
    const line_length = std.mem.indexOfPos(u8, input, 0, "\n").?;

    const source_position = std.mem.indexOfPos(u8, input, 0, "S").?;
    var visited = std.AutoHashMap(usize, void).init(allocator);
    defer visited.deinit();

    var splitters = std.AutoHashMap(usize, void).init(allocator);
    defer splitters.deinit();

    var queue = Queue(usize).init(allocator);
    try queue.enqueue(source_position);

    outer: while (queue.dequeue()) |beam| {
        var position = beam;

        // find next splitter
        while (true) {
            position += line_length + 1;
            if (position >= input.len) continue :outer;
            if (input[position] == '^') break;
        }

        // only count new splitters
        if (!splitters.contains(position)) {
            try splitters.put(position, {});
        }

        const left_pos = position - 1;
        const right_pos = position + 1;

        if (!visited.contains(left_pos)) {
            try visited.put(left_pos, {});
            try queue.enqueue(left_pos);
        }

        if (!visited.contains(right_pos)) {
            try visited.put(right_pos, {});
            try queue.enqueue(right_pos);
        }
    }
    return splitters.count();
}

fn countTimelines(input: []const u8, source: usize, line_length: usize, cache: *std.AutoHashMap(usize, usize)) !usize {
    if (cache.get(source)) |count| {
        return count;
    }

    var position = source;
    // find next splitter
    while (true) {
        position += line_length + 1;

        if (position >= input.len) {
            // timeline reaches end
            try cache.put(source, 1);
            return 1;
        }

        if (input[position] == '^') break;
    }

    const left = try countTimelines(input, position - 1, line_length, cache);
    const right = try countTimelines(input, position + 1, line_length, cache);

    try cache.put(source, left + right);
    return left + right;
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    const line_length = std.mem.indexOfPos(u8, input, 0, "\n").?;

    const source_position = std.mem.indexOfPos(u8, input, 0, "S").?;
    var cache = std.AutoHashMap(usize, usize).init(allocator);
    defer cache.deinit();

    return try countTimelines(input, source_position, line_length, &cache);
}

pub fn main() !void {
    const input = @embedFile("data/inputs/07.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(21, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(40, part2(example, gpa));
}
