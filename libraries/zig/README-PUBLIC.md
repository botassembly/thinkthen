# ThinkThen for Zig

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-zig-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with your platform. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```zig
const std = @import("std");
const tt = @import("thinkthen");
pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => |failure| { defer tt.releaseFailure(allocator, failure); return error.EngineBuild; },
    };
    defer engine.deinit();
    var refund: ?bool = null;
    var records: u64 = 0;
    for ([_][]const u8{ "Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back." }, 0..) |text, index| {
        var call = switch (try tt.calls.decide(&engine, tt.calls.Arguments(tt.inputs.RequestInputText){
            .question = .{ .text = .{ .text = "Does the customer ask for a refund?" } },
            .input = .{ .text = text },
        })) {
            .ok => |value| value,
            .failed => |failure| { defer tt.releaseFailure(allocator, failure); return error.CallFailed; },
        };
        defer call.deinit();
        if (index == 0) refund = if (call.value) |value| value.boolean else null;
        records += call.facts.?.records;
    }
    std.debug.print("refund={any} records={d}\n", .{ refund, records });
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
