const std = @import("std");
const aoc = @import("advent-of-code");

const example = @embedFile("data/examples/10.txt");

const State = struct {
    indicator: u16,
    presses: usize,
    counters: []const u16,

    fn less(context: void, a: State, b: State) std.math.Order {
        _ = context;
        return std.math.order(a.presses, b.presses);
    }
};

const BFSQueue = std.PriorityQueue(State, void, State.less);

fn solve(
    wanted_indicator: u16,
    buttons: []const u16,
    queue: *BFSQueue,
    visited: *std.AutoArrayHashMap(u16, void),
) !usize {
    const counters: []const u16 = &.{};

    try queue.add(.{ .indicator = 0, .presses = 0, .counters = counters });
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
                    .counters = counters,
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

    var buttons = try std.ArrayList(u16).initCapacity(allocator, 10);
    defer buttons.deinit(allocator);

    var lines = std.mem.splitSequence(u8, input[0 .. input.len - 1], "\n");
    while (lines.next()) |line| {
        var indicator_state: u16 = 0;
        buttons.clearRetainingCapacity();

        var position: usize = 1;
        while (line[position] != ']') : (position += 1) {
            if (line[position] == '#') {
                indicator_state |= std.math.shl(u16, 1, position - 1);
            }
        }

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

fn solveLinearSystem(
    allocator: std.mem.Allocator,
    x: []const []const isize,
    target: []const isize,
) !usize {
    const rows = target.len;
    const columns = x.len;

    // we create an augemnted matrix [A | b], where A is built from `x` and `b` is the `target`
    var matrix = try allocator.alloc([]isize, rows);
    defer allocator.free(matrix);
    defer for (matrix) |row| allocator.free(row);

    for (matrix, 0..) |*row, i| {
        // we need an additional column for the augmented matrix
        row.* = try allocator.alloc(isize, columns + 1);
        @memset(row.*, 0);

        // set b vector/ righthand side for Ax = b
        row.*[columns] = target[i];
    }

    for (x, 0..) |column, i| {
        for (column) |value| {
            matrix[@intCast(value)][i] = 1;
        }
    }

    var pivot_columns = try allocator.alloc(?usize, rows);
    defer allocator.free(pivot_columns);
    @memset(pivot_columns, null);

    var free_variables = try std.ArrayList(usize).initCapacity(allocator, rows);
    defer free_variables.deinit(allocator);

    // use gaussian elimination to get a row-echolon form
    var current_row: usize = 0;
    for (0..columns) |col| {
        if (current_row >= rows) {
            // remaining columns are free variables
            try free_variables.append(allocator, col);
            continue;
        }

        var pivot_row = current_row;
        var max_value = @abs(matrix[current_row][col]);

        // find row with highest pivot
        for (current_row + 1..rows) |row| {
            const value = @abs(matrix[row][col]);
            if (value > max_value) {
                max_value = value;
                pivot_row = row;
            }
        }

        if (max_value == 0) {
            // column has no pivot, is a free variable
            try free_variables.append(allocator, col);
            continue;
        }

        // swap current row with pivot
        if (pivot_row != current_row) {
            std.mem.swap([]isize, &matrix[current_row], &matrix[pivot_row]);
        }

        pivot_columns[current_row] = col;

        // eliminate column
        for (current_row + 1..rows) |row| {
            if (matrix[row][col] == 0) continue;

            const factor_num = matrix[row][col];
            const factor_den = matrix[current_row][col];

            for (col..columns + 1) |k| {
                matrix[row][k] = matrix[row][k] * factor_den - matrix[current_row][k] * factor_num;
            }
        }

        current_row += 1;
    }

    var bounds = try allocator.alloc(isize, free_variables.items.len);
    defer allocator.free(bounds);

    for (free_variables.items, 0..) |col, i| {
        bounds[i] = std.math.maxInt(isize);
        for (x[col]) |target_index| {
            bounds[i] = @min(bounds[i], target[@intCast(target_index)]);
        }
    }

    const initial_assignments = try allocator.alloc(isize, free_variables.items.len);
    defer allocator.free(initial_assignments);

    const solution = try allocator.alloc(isize, columns);
    defer allocator.free(solution);

    var min_cost: ?isize = null;

    try exploreSolutions(
        matrix,
        solution,
        0,
        initial_assignments,
        free_variables.items,
        bounds,
        &min_cost,
    );

    return @intCast(min_cost.?);
}

fn exploreSolutions(
    matrix: []const []const isize,
    solution: []isize,
    assignment_index: usize,
    assignments: []isize,
    free_columns: []usize,
    bounds: []const isize,
    min_cost: *?isize,
) !void {
    // if we assigned as many variables as needed, solve the system
    if (assignment_index == free_columns.len) {
        @memset(solution, 0);

        for (free_columns, 0..) |col, i| {
            solution[col] = assignments[i];
        }

        const result = backwardsSubstition(matrix, solution) orelse return;
        if (min_cost.* == null or min_cost.*.? > result) {
            min_cost.* = result;
        }

        return;
    }

    // we still have free variables, explore their values
    const upper_bound = @abs(bounds[assignment_index]);

    // prune early, if we're already worse than our best solution so-far
    if (min_cost.*) |cost| {
        var partial_cost: isize = 0;
        for (assignments[0..assignment_index]) |val| {
            partial_cost += val;
        }

        if (partial_cost >= cost) {
            return;
        }
    }

    for (0..upper_bound + 1) |value| {
        assignments[assignment_index] = @intCast(value);
        try exploreSolutions(
            matrix,
            solution,
            assignment_index + 1,
            assignments,
            free_columns,
            bounds,
            min_cost,
        );
    }
}

fn backwardsSubstition(matrix: []const []const isize, solution: []isize) ?isize {
    const columns = matrix[0].len - 1;
    const rows = matrix.len;

    var row_index: usize = rows;
    while (row_index > 0) {
        row_index -= 1;

        // find the leading coefficient
        const leading_column: usize = for (0..columns) |col| {
            if (matrix[row_index][col] != 0) {
                break col;
            }

            // we do not have any unsolvable case (e.g. `0 0 0 | 3`), so we don't handle this here
            // however we could have zero row (e.g. `0 0 0  | 0`), which can be ignored
        } else continue;

        const col = leading_column;

        var rhs = matrix[row_index][columns];
        for (col + 1..columns) |k| {
            rhs -= matrix[row_index][k] * solution[k];
        }

        if (@rem(rhs, matrix[row_index][col]) != 0) {
            return null;
        }

        const value = @divTrunc(rhs, matrix[row_index][col]);
        if (value < 0) {
            // negative values are no allowed per problem description
            return null;
        }

        solution[col] = value;
    }

    // return the sum of the solution, which is the fewest overall presses
    var total: isize = 0;
    for (solution) |presses| total += presses;

    return total;
}

fn debug_print_matrix(matrix: []const []const isize) void {
    std.debug.print("Matrix: \n", .{});
    for (matrix) |row| {
        for (row, 0..) |value, i| {
            if (i == row.len - 1) {
                std.debug.print("| ", .{});
            }
            std.debug.print("{d} ", .{value});
        }
        std.debug.print("\n", .{});
    }
}

pub fn part2(input: []const u8, allocator: std.mem.Allocator) !usize {
    var total_interactions: usize = 0;

    var buttons = try std.ArrayList([]isize).initCapacity(allocator, 10);
    defer buttons.deinit(allocator);

    var joltage_counters = try std.ArrayList(isize).initCapacity(allocator, 16);
    defer joltage_counters.deinit(allocator);

    var lines = std.mem.splitSequence(u8, input[0 .. input.len - 1], "\n");
    while (lines.next()) |line| {
        buttons.clearRetainingCapacity();
        joltage_counters.clearRetainingCapacity();

        var position = std.mem.indexOf(u8, line, "]").? + 2;

        var button = try std.ArrayList(isize).initCapacity(allocator, 10);
        defer for (buttons.items) |mem| allocator.free(mem);

        while (line[position] != '{') : (position += 1) {
            const value = line[position];
            switch (value) {
                '(' => button.clearRetainingCapacity(),
                '0'...'9' + 1 => button.append(allocator, value - '0') catch unreachable,
                ')' => buttons.append(allocator, try button.toOwnedSlice(allocator)) catch unreachable,
                else => {},
            }
        }

        var joltage_it = std.mem.splitSequence(u8, line[position + 1 .. line.len - 1], ",");
        while (joltage_it.next()) |str| {
            const value = try std.fmt.parseInt(isize, str, 10);
            try joltage_counters.append(allocator, value);
        }

        const solution = try solveLinearSystem(allocator, buttons.items, joltage_counters.items);
        total_interactions += solution;
    }

    return total_interactions;
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
    try std.testing.expectEqual(33, part2(example, gpa));
}
