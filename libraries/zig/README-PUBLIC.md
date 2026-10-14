# ThinkThen for Zig

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-zig-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with your platform. Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```zig
const std = @import("std"); const tt = @import("thinkthen");
pub fn main() !void {
    const allocator = std.heap.page_allocator;
    var engine = switch (try tt.Engine.init(allocator)) {
        .ok => |value| value,
        .failed => |failure| { defer tt.releaseFailure(allocator, failure); return error.EngineBuild; },
    };
    defer engine.deinit();
    const teams = tt.inputs.AuthoredOptions{ .object = .{ .entries = &.{
        .{ .key = "billing", .value = .{ .string = "Invoices, fees, and refunds." } },
        .{ .key = "shipping", .value = .{ .string = "Parcels and delivery." } },
        .{ .key = "account", .value = .{ .string = "Logins and passwords." } },
    } } };
    for ([_][]const u8{ "Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back." }) |text| {
        var call = try tt.calls.decide(&engine, .{ .question = .{ .text = .{ .text = "Does the customer ask for a refund?" } }, .input = .{ .text = .{ .text = text } } }); defer call.deinit();
        const refund = try call.value(); std.debug.print("{any}\n", .{refund});
    }
    for ([_][]const u8{ "Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password." }) |text| {
        var call = try tt.calls.choose(&engine, .{ .question = .{ .definition = .{ .value = .{ .Authored_choose = .{ .choose = .{ .string = "Which team owns this?" }, .options = teams } } } }, .input = .{ .text = .{ .text = text } } }); defer call.deinit();
        const team = try call.value(); std.debug.print("{s}\n", .{team orelse "not sure"});
    }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
