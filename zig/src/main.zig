const std = @import("std");
const Allocator = std.mem.Allocator;

const MAX_FILE_SIZE: usize = 100 * 1_000_000;

const Op = enum {
    Add,
    Sub,
    Right,
    Left,
    Loop,
    Print,
};

const Instruction = union {
    op: Op,
    loop: []Instruction,
};

pub fn main() anyerror!void {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const gpa = general_purpose_allocator.allocator();

    std.log.err("vm.Instruction={d}\n*vm.Instruction={d}", .{ @sizeOf(Instruction), @sizeOf(*Instruction) });
    if (std.os.argv.len < 2) {
        std.log.warn("missing source file argument", .{});
        return;
    }

    var fileName = std.os.argv[1];
    var file = try std.fs.cwd().openFileZ(fileName, .{ .read = true });
    var code = try file.readToEndAlloc(gpa, MAX_FILE_SIZE);
    defer gpa.free(code);

    var comp = compiler.new(gpa, code);
    var inst = try comp.compile();

    // try disassemble(inst);

    var engine = vm.new();
    try engine.run(std.io.getStdOut().writer(), inst);
}

fn disassemble(code: []Instruction) anyerror!void {
    const out = std.io.getStdOut().writer();
    var i: usize = 0;
    while (i < code.len) {
        var op = code[i].op;
        switch (op) {
            Op.Add => try out.print("ADD\n", .{}),
            Op.Sub => try out.print("SUB\n", .{}),
            Op.Right => try out.print("RIGHT\n", .{}),
            Op.Left => try out.print("LEFT\n", .{}),
            Op.Print => try out.print("PRINT\n", .{}),
            Op.Loop => {
                try out.print("LOOP\n", .{});
                i += 1;
                try disassemble(code[i].loop);
                try out.print("END LOOP\n", .{});
            },
        }
        i += 1;
    }
}

const compiler = struct {
    source: []const u8,
    index: usize,
    alloc: Allocator,

    fn new(alloc: Allocator, source: []const u8) compiler {
        return compiler{
            .source = source,
            .alloc = alloc,
            .index = 0,
        };
    }

    fn next(self: *compiler) ?u8 {
        if (self.index < self.source.len) {
            self.index += 1;
            return self.source[self.index - 1];
        }
        return null;
    }

    fn compile(self: *compiler) anyerror![]Instruction {
        var list = std.ArrayList(Instruction).init(self.alloc);
        while (self.next()) |char| {
            switch (char) {
                '+' => try list.append(Instruction{ .op = Op.Add }),
                '-' => try list.append(Instruction{ .op = Op.Sub }),
                '>' => try list.append(Instruction{ .op = Op.Right }),
                '<' => try list.append(Instruction{ .op = Op.Left }),
                '.' => try list.append(Instruction{ .op = Op.Print }),
                '[' => {
                    try list.append(Instruction{ .op = Op.Loop });
                    var loop: []Instruction = try self.compile();
                    try list.append(Instruction{ .loop = loop });
                },
                ']' => {
                    return list.items;
                },
                else => {},
            }
        }
        return list.items;
    }
};

const vm = struct {
    mem: [30000]u8,
    mp: usize,

    fn new() vm {
        return vm{
            .mem = [_]u8{0} ** 30000,
            .mp = 0,
        };
    }
    fn run(self: *vm, writer: anytype, code: []Instruction) anyerror!void {
        var i: usize = 0;
        while (i < code.len) {
            // std.log.info("i={}, op={}, mem={}, mp={}", .{
            //     i,
            //     code.items[i].op,
            //     self.mem[self.mp],
            //     self.mp,
            // });
            // var buf: [1]u8 = [1]u8{0};
            // _ = try std.io.getStdIn().reader().read(buf);
            switch (code[i].op) {
                Op.Add => self.mem[self.mp] +%= 1,
                Op.Sub => self.mem[self.mp] -%= 1,
                Op.Right => self.mp += 1,
                Op.Left => self.mp -= 1,
                Op.Print => _ = try writer.write(self.mem[self.mp .. self.mp + 1]),
                Op.Loop => {
                    i += 1;
                    while (self.mem[self.mp] != 0) {
                        try self.run(writer, code[i].loop);
                    }
                },
            }
            i += 1;
        }
    }
};
