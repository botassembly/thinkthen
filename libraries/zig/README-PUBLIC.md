# ThinkThen for Zig

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

Download the matching archive from the [release downloads](https://github.com/botassembly/thinkthen/releases), then unpack it: `tar -xzf thinkthen-zig-0.2.0-TARGET.tar.gz`. Replace `TARGET` with your platform. Use Zig 0.15.2 and the packaged `build.zig` to link your program. Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

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
    const question: tt.inputs.RequestQuestion = .{ .text = .{ .text = "Does the customer ask for a refund?" } };
    const messages = [_][]const u8{ "Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back." };
    const expected = [_]bool{ true, false, true };
    const probabilities = [_]f64{ 0.99, 0.01, 0.54 };
    var records: [3]tt.inputs.RequestItem = undefined;
    for (messages, 0..) |text, index| {
        records[index] = .{ .original = .{ .text = .{ .text = text } } };
        var refund = switch (try tt.calls.decide(&engine, tt.calls.Arguments(tt.inputs.RequestInputText){ .question = question, .input = .{ .text = text } })) {
            .ok => |result| result, .failed => |failure| { defer tt.releaseFailure(allocator, failure); return error.CallFailed; },
        };
        defer refund.deinit();
        std.debug.assert(refund.value.?.boolean == expected[index] and refund.probability.? == probabilities[index]);
    }
    var selected = switch (try tt.calls.filter(&engine, tt.calls.Arguments(tt.inputs.RequestInputRecords){ .question = question, .input = .{ .items = &records } })) {
        .ok => |result| result, .failed => |failure| { defer tt.releaseFailure(allocator, failure); return error.CallFailed; },
    };
    defer selected.deinit();
    std.debug.assert(selected.value.len == 2 and std.mem.eql(u8, selected.value[0].string, messages[0]) and std.mem.eql(u8, selected.value[1].string, messages[2]));
    std.debug.assert(selected.probability[0].? == 0.99 and selected.probability[1].? == 0.54);
}
```
The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/zig/) for results, failures, record inputs and offline replay.
