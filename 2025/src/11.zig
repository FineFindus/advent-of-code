const std = @import("std");
const aoc = @import("advent-of-code");

const out = ((@as(u24, 'o') << 16) | (@as(u24, 'u') << 8) | 't');

fn countPaths(
    source: u24,
    target: u24,
    graph: *const std.AutoArrayHashMap(u24, std.ArrayList(u24)),
    visited: *std.AutoHashMap(u24, void),
    cache: *std.AutoHashMap(u24, usize),
    count: usize,
) !usize {
    if (cache.get(source)) |cached_value| {
        return cached_value;
    }

    if (source == target) {
        return count + 1;
    }

    // out does not have any out-going edges
    if (source == out) return 0;

    // mark current node as visited
    try visited.put(source, {});
    var sum = count;

    for (graph.get(source).?.items) |node| {
        if (!visited.contains(node)) {
            sum += try countPaths(node, target, graph, visited, cache, count);
        }
    }

    _ = visited.remove(source);
    try cache.put(source, sum);

    return sum;
}

fn parseGraph(input: []const u8, allocator: std.mem.Allocator) !std.AutoArrayHashMap(u24, std.ArrayList(u24)) {
    var graph = std.AutoArrayHashMap(u24, std.ArrayList(u24)).init(allocator);

    var it = std.mem.splitSequence(u8, input[0 .. input.len - 1], "\n");
    while (it.next()) |line| {
        var options = try std.ArrayList(u24).initCapacity(allocator, 8);

        var options_it = std.mem.splitSequence(u8, line[5..], " ");
        while (options_it.next()) |option| {
            const value = ((@as(u24, option[0]) << 16) | (@as(u24, option[1]) << 8) | option[2]);
            try options.append(allocator, value);
        }

        const start = ((@as(u24, line[0]) << 16) | (@as(u24, line[1]) << 8) | line[2]);
        try graph.put(start, options);
    }
    return graph;
}

pub fn part1(input: []const u8, allocator: std.mem.Allocator) !usize {
    var graph = try parseGraph(input, allocator);
    defer {
        var outputs_it = graph.iterator();
        while (outputs_it.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        graph.deinit();
    }

    var visited = std.AutoHashMap(u24, void).init(allocator);
    defer visited.deinit();

    const source = ((@as(u24, 'y') << 16) | (@as(u24, 'o') << 8) | 'u');

    var cache = std.AutoHashMap(u24, usize).init(allocator);
    defer cache.deinit();

    const paths = try countPaths(source, out, &graph, &visited, &cache, 0);
    return paths;
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    var graph = try parseGraph(input, allocator);
    defer {
        var outputs_it = graph.iterator();
        while (outputs_it.next()) |entry| {
            entry.value_ptr.deinit(allocator);
        }
        graph.deinit();
    }

    var visited = std.AutoHashMap(u24, void).init(allocator);
    defer visited.deinit();

    var cache = std.AutoHashMap(u24, usize).init(allocator);
    defer cache.deinit();

    const source = ((@as(u24, 's') << 16) | (@as(u24, 'v') << 8) | 'r');
    const fft = ((@as(u24, 'f') << 16) | (@as(u24, 'f') << 8) | 't');
    const dac = ((@as(u24, 'd') << 16) | (@as(u24, 'a') << 8) | 'c');

    var paths = try countPaths(source, fft, &graph, &visited, &cache, 0);

    cache.clearRetainingCapacity();
    visited.clearRetainingCapacity();
    paths *= try countPaths(fft, dac, &graph, &visited, &cache, 0);

    cache.clearRetainingCapacity();
    visited.clearRetainingCapacity();
    paths *= try countPaths(dac, out, &graph, &visited, &cache, 0);

    return paths;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/11.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const example = @embedFile("data/examples/11.txt");

    const gpa = std.testing.allocator;
    try std.testing.expectEqual(5, part1(example, gpa));
}

test "part two" {
    const example = @embedFile("data/examples/11_2.txt");

    const gpa = std.testing.allocator;
    try std.testing.expectEqual(2, part2(example, gpa));
}
