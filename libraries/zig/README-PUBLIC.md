# ThinkThen for Zig

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-zig-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with your platform. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

The [complete example](https://github.com/botassembly/thinkthen/blob/main/libraries/zig/examples/decide.zig) handles engine setup and cleanup. Each call below waits and returns an owned typed answer.

```zig
var decision = try tt.calls.decide(&engine, .{
    .question = .{ .text = .{ .text = "Does this ask for a refund?" } },
    .input = .{ .text = .{ .text = "Please refund my order." } },
});
defer decision.deinit();
const yes = try decision.value();
var selected = try tt.calls.filter(&engine, .{
    .options = .{ .batch = .{ .integer = 1 } },
    .question = .{ .text = .{ .text = "Does this ask for a refund?" } },
    .input = .{ .records = .{ .items = &.{
        .{ .original = .{ .text = .{ .text = "Please refund my order." } } },
        .{ .original = .{ .text = .{ .text = "The package arrived." } } },
        .{ .original = .{ .text = .{ .text = "Cancel and refund this order." } } },
    } } },
});
defer selected.deinit();
const records = try selected.value();
std.debug.print("refund={any} records={d}\n", .{ yes, records.len });
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
