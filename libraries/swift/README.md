# ThinkThen for Swift

Call a decision model from Swift and read its typed answer in one line. The ten functions are `decide`, `choose`, `tag`, `score`, `filter`, `rank`, `find`, `annotate`, `recognize` and `relate`.

```swift
import ThinkThen

let client = try Client()
defer { client.close() }
let question = Question.text("Does the customer ask for a refund?")
let message = "Please refund my order. It arrived broken."
let judgment = try await client.decide(question, input: message)
let isRefund = judgment.value
let refundProbability = judgment.probability
precondition(isRefund == .bool(true) && refundProbability == 0.99)
```

A scalar text or JSON input returns one value. `InputRecords`, `InputUnits` and `InputSource` return lists for `decide`, `choose`, `tag` and `score`. Use the concrete input type to select the return type. A score is `Double`; a list of scores is `[Double]`. A decision preserves authored readings as `DataValue`, including booleans, objects and explicit null. A missing decision or choice means not sure. Failed calls throw instead of returning an answer.

Keep the call when you need more than the value. `call.probability` holds an optional answer probability. Record decisions return a list of optional probabilities in input order. Aggregate functions retain their detailed probabilities in `call.results`. It is distinct from a score's value. `call.results` retains the complete typed result records. `call.facts` retains final counts, usage and model information. Each value owns its data after the client closes.

`Question.text` selects literal text. `.definition(QuestionDefinition(value:))` builds an authored question with generated types. `.file`, `.name` and `.reference` select saved questions. Use `OrderedMap` for authored options, recognize kinds and annotate questions. The [executable example](Examples/main.swift) shows typed questions for all ten functions.

`InputRecords` keeps caller records in order. `Input.files` selects native file readers and provenance. Images use `.bytes(data, media:)` or `.file(path, media:)`; the engine checks the selected model and function. `DataValue` retains caller JSON values and exact number literals. `Presence` distinguishes an absent field, explicit null and a value.

## Install

Unpack the matching [release archive](https://github.com/botassembly/thinkthen/releases). Add the unpacked SwiftPM package as a dependency and use `.product(name: "ThinkThen", package: "thinkthen-swift")`. The package includes its native library. Consumers need no Rust build or native path variable. Deploy SwiftPM's resource bundle beside Linux executables. Apple packages include `CThinkThen.xcframework` and require macOS 15 or later.

Set a hosted key with `THINKTHEN_API_KEY`. [Backend setup](https://thinkthen.dev/install/backends/) covers Jev, SystemOne, OpenAI and local models. The library is MIT licensed. Hosted models charge per token; local models use your computer.

## Advanced calls

`EngineSettings` configures one backend per client. `client.plan` returns a typed plan without sending. `client.start` opens a bounded `Session`; one producer pushes descriptors and finishes input while one reader reads packets. `client.execute(request, feed:)` accepts an `AsyncThrowingStream`. `Session.cancel()` signals cancellation; `Session.close()` releases native ownership. Use `defer` to close clients.

`SessionFailure` retains the generated failure, completed packets and final facts. `SessionBoundaryFailure` retains the safe native admission diagnostic. Task cancellation throws `CancellationError`. `usagePersistence()` observes usage storage; `finishUsageStatus()` finishes pending usage writes. Run finalization on a background executor when needed.

## Build and check

The generated input, result and native bridge checks run through `sdlc/generators/results/generate.py`. `check.sh` installs a versioned package into an unrelated consumer and checks typed answers, failure ownership, cancellation and saved responses. `THINKTHEN_TEST_PROFILE=full` selects the full shared inventory.
