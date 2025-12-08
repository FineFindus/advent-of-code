const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/08.txt");

fn DSU(N: usize) type {
    return struct {
        const Self = @This();

        parent: [N]usize,
        size: [N]usize,

        fn init() Self {
            var parent = [_]usize{0} ** N;
            var size = [_]usize{0} ** N;

            for (0..N) |i| {
                parent[i] = i;
                size[i] = 1;
            }

            return .{
                .parent = parent,
                .size = size,
            };
        }

        /// Finds the representative (root) of the set that includes `i`.
        fn find(self: *Self, i: usize) usize {
            if (self.parent[i] != i) {
                self.parent[i] = self.find(self.parent[i]);
            }
            return self.parent[i];
        }

        /// Merges the set set that includes element `x` and the set that includes element `y`.
        fn unite(self: *Self, x: usize, y: usize) void {
            const root_x = self.find(x);
            const root_y = self.find(y);
            if (root_x == root_y) return;

            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        fn getComponentSizes(self: *Self, allocator: std.mem.Allocator) ![]usize {
            var sizes = std.AutoHashMap(usize, usize).init(allocator);
            defer sizes.deinit();

            for (0..self.parent.len) |i| {
                const root = self.find(i);
                try sizes.put(root, self.size[root]);
            }

            var result = try allocator.alloc(usize, sizes.count());
            var it = sizes.valueIterator();
            var idx: usize = 0;
            while (it.next()) |size| : (idx += 1) {
                result[idx] = size.*;
            }

            std.sort.pdq(usize, result, {}, comptime std.sort.desc(usize));
            return result;
        }
    };
}

const Point = struct {
    x: isize,
    y: isize,
    z: isize,

    fn distance(lhs: Point, rhs: Point) isize {
        return (lhs.x - rhs.x) * (lhs.x - rhs.x) + (lhs.y - rhs.y) * (lhs.y - rhs.y) + (lhs.z - rhs.z) * (lhs.z - rhs.z);
    }
};

const Edge = struct {
    i: usize,
    j: usize,
    dist: isize,

    fn lessThan(_: void, a: Edge, b: Edge) std.math.Order {
        return std.math.order(a.dist, b.dist);
    }
};

fn parsePoints(input: []const u8, allocator: std.mem.Allocator) !std.ArrayList(Point) {
    var points = try std.ArrayList(Point).initCapacity(allocator, 1000);

    var it = std.mem.splitSequence(u8, input, "\n");
    var length: usize = 0;
    while (it.next()) |line| : (length += 1) {
        if (line.len == 0) break;
        var coord_it = std.mem.splitSequence(u8, line, ",");
        const point = Point{
            .x = try std.fmt.parseInt(isize, coord_it.next().?, 10),
            .y = try std.fmt.parseInt(isize, coord_it.next().?, 10),
            .z = try std.fmt.parseInt(isize, coord_it.next().?, 10),
        };
        try points.append(allocator, point);
    }

    return points;
}

fn createEdges(points: []Point, allocator: std.mem.Allocator) !std.PriorityDequeue(Edge, void, Edge.lessThan) {
    var heap = std.PriorityDequeue(Edge, void, Edge.lessThan).init(allocator, {});

    for (0..points.len) |i| {
        for (i + 1..points.len) |j| {
            try heap.add(Edge{
                .i = i,
                .j = j,
                .dist = points[i].distance(points[j]),
            });
        }
    }

    return heap;
}

pub fn part1(input: []const u8, allocator: std.mem.Allocator) !usize {
    var points = try parsePoints(input, allocator);
    defer points.deinit(allocator);

    var heap = try createEdges(points.items, allocator);
    defer heap.deinit();

    var dsu = DSU(1000).init();

    //HACK: this should be only 10 for testing
    const pairs_to_process: usize = if (points.items.len == 20) 10 else 1000;
    var processed: usize = 0;

    while (processed < pairs_to_process and heap.count() > 0) : (processed += 1) {
        const edge = heap.removeMin();
        dsu.unite(edge.i, edge.j);
    }

    const sizes = try dsu.getComponentSizes(allocator);
    defer allocator.free(sizes);

    return sizes[0] * sizes[1] * sizes[2];
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    var points = try parsePoints(input, allocator);
    defer points.deinit(allocator);

    var heap = try createEdges(points.items, allocator);
    defer heap.deinit();

    var dsu = DSU(1000).init();

    var last_edge: ?Edge = null;
    while (heap.removeMinOrNull()) |edge| {
        if (dsu.find(edge.i) != dsu.find(edge.j)) {
            dsu.unite(edge.i, edge.j);
            last_edge = edge;
        }
    }

    const x_1: usize = @intCast(points.items[last_edge.?.i].x);
    const x_2: usize = @intCast(points.items[last_edge.?.j].x);
    return x_1 * x_2;
}

pub fn main() !void {
    const input = @embedFile("data/inputs/08.txt");

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    try aoc.solve(input, 1, part1, allocator);
    try aoc.solve(input, 2, part2, allocator);
}

test "part one" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(40, part1(example, gpa));
}

test "part two" {
    const gpa = std.testing.allocator;
    try std.testing.expectEqual(25272, part2(example, gpa));
}
