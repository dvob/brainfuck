const std = @import("std");

pub const Tag = enum {
    Inc,
    Dec,
    IncRef,
    DecRef,
    Jump,
    CondJump,
    Print,
};

pub const Instruction = union(Tag) {
    Inc: u8,
    Dec: u8,
    IncRef: usize,
    DecRef: usize,
    Jump: usize,
    CondJump: usize,
    Print: void,
};

pub const Vm = struct {
    mem: [30000]u8,
    mp: usize,
    ip: usize,
    code: []const Instruction,

    pub fn new(code: []const Instruction) Vm {
        return Vm{
            .mem = [_]u8{0} ** 30000,
            .mp = 0,
            .ip = 0,
            .code = code,
        };
    }

    pub fn run(self: *Vm) anyerror!void {
        // var out = std.io.bufferedWriter(std.io.getStdOut().writer());
        while (self.ip < self.code.len) {
            // std.log.info("ip={}, op={}, mp={}, mem={}", .{
            //     self.ip,
            //     self.code[self.ip],
            //     self.mp,
            //     self.mem[self.mp],
            // });
            // _ = try std.io.getStdIn().read(&buf);
            switch (self.code[self.ip]) {
                Tag.Inc => |v| self.mem[self.mp] +%= v,
                Tag.Dec => |v| self.mem[self.mp] -%= v,
                Tag.IncRef => |v| self.mp += v,
                Tag.DecRef => |v| self.mp -= v,
                Tag.Jump => |v| {
                    self.ip = v;
                    continue;
                },
                Tag.CondJump => |v| {
                    if (self.mem[self.mp] == 0) {
                        self.ip = v;
                        continue;
                    }
                },
                Tag.Print => {
                    _ = try std.io.getStdOut().write(self.mem[self.mp .. self.mp + 1]);
                    // _ = try out.write(self.mem[self.mp .. self.mp + 1]);
                },
            }
            self.ip += 1;
        }
    }
};
