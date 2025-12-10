const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/10.txt");

const State = struct {
    indicator: u16,
    presses: usize,
};

const BFSQueue = std.PriorityQueue(State, void, struct {
    fn less(context: void, a: State, b: State) std.math.Order {
        _ = context;
        return std.math.order(a.presses, b.presses);
    }
}.less);

fn solve(
    wanted_indicator: u16,
    buttons: []const u16,
    queue: *BFSQueue,
    visited: *std.AutoArrayHashMap(u16, void),
) !usize {
    try queue.add(.{ .indicator = 0, .presses = 0 });
    try visited.put(0, {});

    while (queue.removeOrNull()) |state| {
        if (state.indicator == wanted_indicator) {
            return state.presses;
        }

        for (buttons) |button| {
            const new_indicator = state.indicator ^ button;
            if (!visited.contains(new_indicator)) {
                visited.put(new_indicator, {}) catch unreachable;
                queue.add(.{
                    .indicator = new_indicator,
                    .presses = state.presses + 1,
                }) catch unreachable;
            }
        }
    }

    unreachable;
}

pub fn part1(input: []const u8, allocator: std.mem.Allocator) !usize {
    var total_interactions: usize = 0;

    var queue = BFSQueue.init(allocator, {});
    defer queue.deinit();

    var visited = std.AutoArrayHashMap(u16, void).init(allocator);
    defer visited.deinit();

    var lines = std.mem.splitSequence(u8, input[0 .. input.len - 1], "\n");
    while (lines.next()) |line| {
        var indicator_state: u16 = 0;

        var position: usize = 1;
        while (line[position] != ']') : (position += 1) {
            if (line[position] == '#') {
                indicator_state |= std.math.shl(u16, 1, position - 1);
            }
        }

        var buttons = try std.ArrayList(u16).initCapacity(allocator, 10);
        defer buttons.deinit(allocator);

        var button_wiring: u16 = 0;
        while (line[position] != '{') : (position += 1) {
            const value = line[position];
            switch (value) {
                '(' => button_wiring = 0,
                '0'...'9' + 1 => button_wiring |= std.math.shl(u16, 1, value - '0'),
                ')' => buttons.append(allocator, button_wiring) catch unreachable,
                else => {},
            }
        }

        total_interactions += try solve(indicator_state, buttons.items, &queue, &visited);
        queue.clearRetainingCapacity();
        visited.clearRetainingCapacity();
    }

    return total_interactions;
}

pub fn part2(_: []const u8, _: std.mem.Allocator) !usize {
    return 0;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/10.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(7, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(0, part2(example, gpa));
}
