const std = @import("std");

const vm = @import("vm.zig");

const MAX_FILE_SIZE: usize = 100 * 1_000_000;

pub fn main() anyerror!void {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = general_purpose_allocator.allocator();

    if (std.os.argv.len < 2) {
        std.log.warn("missing source file argument", .{});
        return;
    }

    var file = std.os.argv[1];

    var f = try std.fs.cwd().openFileZ(file, .{ .read = true });
    var code = try f.readToEndAlloc(gpa, MAX_FILE_SIZE);
    defer gpa.free(code);
    var inst = std.ArrayList(vm.Instruction).init(gpa);
    defer inst.deinit();

    _ = try compile(code, &inst);
    // for (inst.items) |item, index| {
    //     std.debug.print("{}: {}\n", .{ index, item });
    // }

    var engine = vm.Vm.new(inst.items);
    try engine.run();
}

fn compile(code: []const u8, inst: *std.ArrayList(vm.Instruction)) anyerror!usize {
    var i: usize = 0;
    while (i < code.len) {
        switch (code[i]) {
            '+' => try inst.append(vm.Instruction{ .Inc = 1 }),
            '-' => try inst.append(vm.Instruction{ .Dec = 1 }),
            '>' => try inst.append(vm.Instruction{ .IncRef = 1 }),
            '<' => try inst.append(vm.Instruction{ .DecRef = 1 }),
            '.' => try inst.append(vm.Instruction.Print),
            '[' => {
                try inst.append(vm.Instruction{ .CondJump = 0 });
                var addr = inst.items.len - 1;
                i += (try compile(code[i + 1 ..], inst)) + 1;
                try inst.append(vm.Instruction{ .Jump = addr });
                inst.items[addr] = vm.Instruction{ .CondJump = inst.items.len };
            },
            ']' => {
                return i;
            },
            else => {},
        }
        i += 1;
    }
    return i;
}
