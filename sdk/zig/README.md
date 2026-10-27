# ThinkThen for Zig

Use Zig 0.17.0. Run `zig fetch --save=thinkthen https://github.com/botassembly/thinkthen/releases/download/v0.2.0/thinkthen-zig-0.2.0-linux-x86_64.tar.gz`. Choose the archive for your platform. Add the dependency module to your executable and install its native library with the [consumer build](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/zig/tests/build.zig).

The example uses the stub backend and needs no key or service.

```zig
const std = @import("std");
const tt = @import("thinkthen");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var arena = std.heap.ArenaAllocator.init(allocator);
    defer arena.deinit();
    var diagnostic: tt.Diagnostic = .{};
    var client = try tt.Client.init(.{ .backend = .stub }, &diagnostic);
    defer client.deinit();
    const records = [_]tt.Record{.{ .text = "evidence" }};
    var call = try client.decide(allocator, "Ready?",
        .{ .records = &records }, .{}, &diagnostic);
    defer call.deinit();
    while (try call.next(arena.allocator(), &diagnostic)) |row| {
        switch (row.outcome) {
            .answered => |value| std.debug.print("answer: {}\n", .{value.answer}),
            .unsure => std.debug.print("not sure\n", .{}),
            .failed => |failure| std.debug.print("{s}: {s}\n", .{failure.name(), failure.message}),
        }
        _ = arena.reset(.free_all);
    }
}
```

[API reference](https://github.com/botassembly/thinkthen/blob/v0.2.0/sdk/zig/src/thinkthen.zig).
