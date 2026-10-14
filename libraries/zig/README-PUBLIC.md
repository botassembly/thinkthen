# ThinkThen for Zig

ThinkThen answers typed questions about text. Ask for a yes or no, a label, a number, or records selected and ranked by meaning. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

```sh
tar -xzf thinkthen-zig-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with your platform. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

This fragment reads one typed answer; the [complete example](https://github.com/botassembly/thinkthen/blob/main/libraries/zig/examples/decide.zig) handles setup and cleanup.

```zig
// Inside the packet branch of an owned decision session:
var packet = value;
defer packet.deinit();
if (packet.failure()) |_| return error.DecisionFailed;
if (packet.value().* == .observation) {
    if (packet.answers().decide()) |answer| {
        std.debug.print("{any}\n", .{answer});
    } else |err| if (err != error.WrongAnswer) return err;
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
