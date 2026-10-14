# ThinkThen for Zig

The source package bundles the matching native engine for x86_64 Linux GNU and requires Zig 0.15.2. `build.zig` links that engine for downstream callers.

Construct an engine with `Engine.init(allocator)` or `Engine.initWithSettings(allocator, inputs.EngineSettings)`. Construction returns `.ok` or an owned `.failed` value. A failure has a native kind, message, retry signal and optional typed `facts`. Release it with `releaseFailure(allocator, failure)`.

The ten `calls.FUNCTION(engine, arguments)` functions block until completion. Supply `calls.Arguments(Input)` with a concrete generated input payload, such as `inputs.RequestInputText` or `inputs.RequestInputRecords`. The input type determines whether record-oriented functions return one value or a slice. A records payload retains a slice even when it contains only one record. `filter` and `rank` always return slices; `find` and `relate` return one result for the whole input.

A call returns `.ok` with an owned result or `.failed` with an owned native failure. Read the result's four fields directly: `value` is the function's typed answer, `probability` is its typed optional probability, `results` retains the complete native answer receipts, and `facts` contains optional terminal facts. Release the result with `deinit()` after every borrowed value finishes. Release a failure with `releaseFailure(allocator, failure)`; it retains its kind, message, retry signal, optional facts and any terminal failure. Zig allocation and shape errors still propagate through `try`.

For a text input, `decide` returns an optional authored yes/no value; a plain question uses its `.boolean` member. `choose` returns an optional string. Their scalar `probability` is `?f64`. Records inputs return aligned slices of values and probabilities. An outer null value means unresolved; an authored `.null_value` inside a present value is a resolved answer. A resolved authored null retains its probability; an unresolved answer has a null probability. Inspect the receipts in `results` when you need the authored reading or the complete model observation.

`tag` returns the selected labels, `score` returns a number, `filter` returns original input values including selected nulls, and `rank` returns original zero-based indices. `find` returns the selected original value or null. `annotate` returns its typed named fields, `recognize` returns typed entities and spans, and `relate` returns typed relation edges. For text inputs, decide and choose have scalar optional probabilities; records inputs have aligned slices. Filter and rank always have probability slices, while find has one optional probability. Tag, score and annotate have null probabilities. Recognize has probabilities aligned to each record's entities, and relate has probabilities aligned to its relation edges. Native receipts and facts remain available for every function.

The [short example](README-PUBLIC.md) decides which messages request refunds and filters the same records. The installed examples under `examples/` show complete call ownership.

Use `Request.start(engine)` or `session.startNamed(engine, function, arguments)` for an owned streaming session. Call `next()` to block for a packet or settlement; call `read()` for an explicit nonblocking read. A feed accepts generated descriptors with `push(allocator, descriptor)`. Use `wait(true)` when its queue is full, then retry. Finish a feed with `finish()` and cancel with `cancel()`.

Each returned packet owns copied Zig values. `packet.value()` borrows the generated `results.SessionPacket` union. `packet.terminal()`, `packet.facts()` and `packet.failure()` return optional typed fields. Missing, null and present fields use generated `Presence(T)` unions. Packets and their nested values survive session and engine destruction. Release every packet with `deinit()` after its borrowers finish.

`packet.answers()` offers typed `decide`, `choose`, `tag`, `score`, `find`, `annotate`, `recognize` and `relate` accessors for normalized observations. `rank` returns original zero-based indices and `filter` returns original input values, including null. Answers borrow the packet and survive engine and session closure. Calling an accessor on another packet returns `error.WrongAnswer`.

`Question.recognize(engine, definition)` and `Question.relate(engine, definition)` admit generated typed definitions through the native engine. `Question.init(engine, role, definition)` uses native saved-question admission. `question.name()` returns its optional borrowed name. The explicit role selects the grammar. Release the question with `deinit()`.

`Request.init(allocator, function, question, input, options)` copies typed arguments independently of the caller. `request.start(engine)` uses native named admission. `request.plan(engine)` previews the same arguments without credentials, cache reads or requests. The returned plan has generated fields under `plan.value` and owns its strings after request and engine destruction. Release both owners with `deinit()`.

```zig
const request = try tt.Request.init(allocator, "decide",
    .{ .text = .{ .text = "Does it pass?" } },
    .{ .text = .{ .text = "Evidence." } },
    .{ .details = false },
);
defer request.deinit();
var plan = try request.plan(&engine);
defer plan.deinit();
const records = plan.value.records;
```

Native preview currently supports fixed atomic decide, choose, tag and score questions. Other functions and dynamic questions return an admission error.

`Engine.usagePersistence()` observes live persistence. `Engine.finishUsageStatus()` drains the engine's current deltas. Both return owned `UsageStatus` values with a state and optional advice. Release them with `status.deinit(allocator)`. Close an engine with `deinit()` after all callers finish.

`check.sh` packages and tests the installed static engine. It exercises admission, preview without sends, retained nested values, failures, persistence and the blocking example. Shared routine cases cover request mapping and complete packets. `THINKTHEN_TEST_PROFILE=full` selects release cases.

Build a matching C archive, unpack it into an absolute native directory and run `zig build -Dnative=/absolute/native/root`. Packaged consumers use the bundled native directory automatically.
