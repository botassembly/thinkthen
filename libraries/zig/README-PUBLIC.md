# ThinkThen for zig

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. Failed calls remain separate from answers.

## Install

Download the matching release archive, then unpack it:

```sh
tar -xzf thinkthen-zig-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded archive. Configure a model backend before running a judgment.

## Example

```zig
const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    const a = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(a)) {
        .ok => |value| value,
        .failed => |failure| {
            defer tt.releaseFailure(a, failure);
            return error.EngineBuild;
        },
    };
    defer engine.deinit();
    var call = try tt.session.decide(&engine, .{
        .question = .{ .text = .{ .text = "Is this a complaint?" } },
        .input = .{ .text = .{ .text = "I demand a refund today" } },
    });
    defer call.deinit();
    while (true) switch (try call.read()) {
        .pending => {
            var ready: u32 = undefined;
            try tt.session.checked(tt.c.thinkthen_session_wait(call.raw, tt.c.THINKTHEN_SESSION_WAIT_SETTLEMENT_V1, 50, &ready));
        },
        .end => break,
        .packet => |value| {
            var packet = value;
            defer packet.deinit();
            if (packet.failure()) |_| return error.DecisionFailed;
            if (packet.facts()) |facts| std.debug.print("records={d} requests={d}\n", .{ facts.records, facts.requests_sent });
        },
    };
}
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
