# ThinkThen for Zig

The source package bundles the matching native engine for x86_64 Linux GNU and requires Zig 0.15.2. `build.zig` links that engine for downstream callers.

Construct an engine with `Engine.init(allocator)` or `Engine.initWithSettings(allocator, inputs.EngineSettings)`. Construction returns `.ok` or an owned `.failed` value. A failure has a native kind, message, retry signal and optional typed `facts`. Release it with `releaseFailure(allocator, failure)`.

The ten `calls.FUNCTION(engine, arguments)` functions accept generated typed question, input and option descriptors. They return an owned session. Call `next()` to block for a packet or settlement; call `read()` only for an explicit nonblocking read. A feed accepts generated descriptors with `push()`. Use `wait(true)` when its queue is full, then retry. Finish a feed with `finish()` and cancel with `cancel()`.

Each returned packet owns copied Zig values. `packet.value()` returns the generated `results.SessionPacket` union. `packet.terminal()`, `packet.facts()` and `packet.failure()` return optional typed fields. Missing, null and present fields use generated `Presence(T)` unions. Packets and their nested values survive session and engine destruction. Release every packet with `deinit()` after its borrowers finish.

`packet.answers()` offers typed `decide`, `choose`, `tag`, `score`, `find`, `annotate`, `recognize` and `relate` accessors for normalized observations. `rank` returns original zero-based indices and `filter` returns original input values, including null. Answers borrow the packet and survive engine and session closure. Calling an accessor on another packet returns `error.WrongAnswer`.

`Question.recognize(engine, definition)` and `Question.relate(engine, definition)` admit generated typed definitions through the native engine. `Question.init(engine, role, definition)` uses native saved-question admission. `question.name()` returns its optional borrowed name. The explicit role selects the grammar. Release the question with `deinit()`.

`Request.init(allocator, call)` copies typed arguments independently of the caller. `request.start(engine)` uses native named admission. `request.plan(engine)` previews the same arguments without credentials, cache reads or requests. The returned plan has generated fields under `plan.value` and owns its strings after request and engine destruction. Release both owners with `deinit()`.

```zig
const request = try tt.Request.init(allocator, .{
    .function = .decide,
    .question = .{ .text = .{ .text = "Does it pass?" } },
    .input = .{ .text = .{ .text = "Evidence." } },
    .options = .{ .details = false },
});
defer request.deinit();
var plan = try request.plan(&engine);
defer plan.deinit();
const records = plan.value.records;
```

Native preview currently supports fixed atomic decide, choose, tag and score questions. Other functions and dynamic questions return an admission error.

`Engine.usagePersistence()` observes live persistence. `Engine.finishUsageStatus()` drains the engine's current deltas. Both return owned `UsageStatus` values with a state and optional advice. Release them with `status.deinit(allocator)`. Close an engine with `deinit()` after all callers finish.

`check.sh` packages and tests the installed static engine. It exercises admission, preview without sends, retained nested values, failures, persistence and the blocking example. Shared routine cases cover request mapping and complete packets. `THINKTHEN_TEST_PROFILE=full` selects release cases.

Build a matching C archive, unpack it into an absolute native directory and run `zig build -Dnative=/absolute/native/root`. Packaged consumers use the bundled native directory automatically.
