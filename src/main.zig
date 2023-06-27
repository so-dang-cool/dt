const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();
const stderr = std.io.getStdErr().writer();

const Token = @import("tokens.zig").Token;

const interpret = @import("interpret.zig");
const RockError = interpret.RockError;
const RockDictionary = interpret.Dictionary;
const RockCommand = interpret.RockCommand;
const RockAction = interpret.RockAction;
const RockMachine = interpret.RockMachine;

const builtins = @import("builtins.zig");

const version = "0.1.1";

const helloFile = @embedFile("test/hello.rock");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);

    var machine = try RockMachine.init(arena.allocator());

    try machine.define("def", "define a new command", .{ .builtin = builtins.def });
    try machine.define(":", "bind variables", .{ .builtin = builtins.colon });
    try machine.define("dup", "duplicate the top value", .{ .builtin = builtins.dup });
    try machine.define("drop", "drop the top value", .{ .builtin = builtins.drop });
    try machine.define("swap", "swap the top two values", .{ .builtin = builtins.swap });
    try machine.define("rot", "rotate the top three values", .{ .builtin = builtins.rot });
    try machine.define("pl", "print a value and a newline", .{ .builtin = builtins.pl });
    try machine.define("+", "add two numeric values", .{ .builtin = builtins.add });
    try machine.define("-", "subtract two numeric values", .{ .builtin = builtins.subtract });
    try machine.define("*", "multiply two numeric values", .{ .builtin = builtins.multiply });
    try machine.define("/", "divide two numeric values", .{ .builtin = builtins.divide });
    try machine.define("%", "modulo two numeric values", .{ .builtin = builtins.modulo });
    try machine.define("abs", "consume a number and produce its absolute value", .{ .builtin = builtins.abs });
    try machine.define(".s", "print the stack", .{ .builtin = builtins.dotS });
    try machine.define("map", "apply a command to all values in a stack", .{ .builtin = builtins.map });
    try machine.define("...", "expand a quote", .{ .builtin = builtins.ellipsis });
    try machine.define("push", "move an item into a quote", .{ .builtin = builtins.push });
    try machine.define("pop", "move the last item of a quote to top of stack", .{ .builtin = builtins.pop });
    try machine.define("enq", "move an item into the first position of a quote", .{ .builtin = builtins.enq });
    try machine.define("deq", "remove an item from the first position of a quote", .{ .builtin = builtins.deq });

    try stderr.print("rock {s}\n", .{version});

    const helloTokens = try Token.parseAlloc(arena.allocator(), helloFile);
    defer helloTokens.deinit();
    for (helloTokens.items) |token| {
        // try stderr.print("Token: {any}\n", .{token});
        try machine.interpret(token);
        // try stderr.print("STATE: {any}\n\n", .{machine.curr.});
    }

    var stop = false;
    while (!stop) {
        try stdout.print("> ", .{});

        const input = try prompt(arena.allocator());

        const tokens = try Token.parseAlloc(arena.allocator(), input);
        for (tokens.items) |token| {
            //try stderr.print("Token: {any}\n", .{token});
            machine.interpret(token) catch |e| {
                switch (token) {
                    .term => |term| try stderr.print("OOPS {s} caused error: {any}\n", .{ term, e }),
                    else => try stderr.print("OOPS {any} caused error: {any}\n", .{ token, e }),
                }
                break;
            };
        }
    }
}

fn prompt(alloc: Allocator) ![]const u8 {
    return stdin.readUntilDelimiterOrEofAlloc(alloc, '\n', 128) catch |err| {
        const message = switch (err) {
            error.StreamTooLong => "Response was too many characters.",
            else => "Unable to read response.",
        };
        try stderr.print("\nERROR: {s} ({any})\n", .{ message, err });
        std.os.exit(1);
    } orelse {
        try stderr.print("\nBye now.\n", .{});
        std.os.exit(0);
    };
}
