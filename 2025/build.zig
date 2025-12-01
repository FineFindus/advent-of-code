const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const download_opt = b.option(bool, "download", "Whether to download the input file") orelse false;
    const day_opt = b.option(u32, "day", "Which Advent of Code day to build/run/test") orelse 0;
    if (day_opt == 0) {
        std.debug.print("-Dday=N is required\n", .{});
        return;
    }

    const template_path = "template/main.zig";
    const day_name = std.fmt.allocPrint(b.allocator, "day{d}", .{day_opt}) catch unreachable;
    const src_file = std.fmt.allocPrint(b.allocator, "src/{s}.zig", .{day_name}) catch unreachable;
    defer b.allocator.free(day_name);
    defer b.allocator.free(src_file);

    const generate_step = b.step("generate", "Generate source file and download input");
    {
        // copy the template and update it to load the correct day
        const contents = std.fs.cwd().readFileAlloc(b.allocator, template_path, 1024) catch unreachable;
        defer b.allocator.free(contents);

        const template = std.mem.replaceOwned(u8, b.allocator, contents, "day", day_name) catch unreachable;
        defer b.allocator.free(template);

        const file = std.fs.cwd().createFile(
            src_file,
            .{},
        ) catch unreachable;
        defer file.close();

        file.writeAll(template) catch unreachable;

        // download inputs
        if (download_opt) {
            const cmd = b.addSystemCommand(&[_][]const u8{
                "aoc",          "download",                                                           "--year",       "2025",
                "-d",           std.fmt.allocPrint(b.allocator, "{d}", .{day_opt}) catch unreachable, "--input-file", std.fmt.allocPrint(b.allocator, "src/data/inputs/{d:0>2}.txt", .{day_opt}) catch unreachable,
                "--input-only",
            });
            generate_step.dependOn(&cmd.step);
        }
    }

    const mod = b.addModule("advent-of-code", .{
        .root_source_file = b.path("src/root.zig"),
        .target = target,
    });

    const exe = b.addExecutable(.{
        .name = "advent-of-code",
        .root_module = b.createModule(.{
            .root_source_file = b.path(src_file),
            .target = target,
            .optimize = optimize,
            .imports = &.{
                .{ .name = "advent-of-code", .module = mod },
            },
        }),
    });
    b.installArtifact(exe);

    const solve_step = b.step("solve", "Solve the selected day");
    const solve_cmd = b.addRunArtifact(exe);
    solve_step.dependOn(&solve_cmd.step);

    // By making the run step depend on the default step, it will be run from the
    // installation directory rather than directly from within the cache directory.
    solve_cmd.step.dependOn(b.getInstallStep());

    // This allows the user to pass arguments to the application in the build
    // command itself, like this: `zig build run -- arg1 arg2 etc`
    if (b.args) |args| {
        solve_cmd.addArgs(args);
    }

    const exe_tests = b.addTest(.{
        .root_module = exe.root_module,
    });
    const run_exe_tests = b.addRunArtifact(exe_tests);

    const test_step = b.step("test", "Run the tests for the selected day");
    test_step.dependOn(&run_exe_tests.step);
}
