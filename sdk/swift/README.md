# Swift SDK

The Swift SDK asks questions about text from your Swift code. Its first function, decide, answers a yes-or-no question with yes (`true`), no (`false`) or not sure.

It needs Swift 6.3 or later, and runs on macOS 15 or later on Apple silicon and on Linux (x86-64 or arm64) with glibc 2.35 or later. Add it to the dependencies in `Package.swift`:

```swift
.package(
    url: "https://github.com/botassembly/thinkthen",
    exact: "0.2.0")
```

Then add `.product(name: "ThinkThen", package: "thinkthen")` to your target. On macOS, also set `platforms: [.macOS("15.0")]`. It carries its own engine. The client reads your key from `THINKTHEN_API_KEY`. [Configuration](https://thinkthen.dev/configuration/) shows how to get one.

## Ask a question

Ask whether a support message requests a refund:

```swift
import ThinkThen

let client = try Client()
let isRefund = try await client.decide(
    "Does the customer request a refund?",
    record: .text("Please refund my order.")).get()
print(isRefund)
```

Output:

```text
true
```

## Not sure and failures

Set a low and a high bar, and decide is not sure between them. `get()` throws `RowError.unsure` when decide is not sure, and `RowError.failed` when ThinkThen could not judge the message, so neither one passes for `false`. Send both to a person.

```swift
import ThinkThen

let client = try Client()
let refundCheck = try await client.decide(
    "Does the customer request a refund?",
    record: .text("I was charged twice for one invoice."),
    options: .init(threshold: .band(low: 0.2, high: 0.8)))
do {
    print(try refundCheck.get())
} catch RowError.unsure(let probability) {
    print("Not sure (\(probability)): ask a person.")
} catch RowError.failed(let failure) {
    print("\(failure.code.rawValue): \(failure.message)")
}
```

Output:

```text
Not sure (0.7): ask a person.
```

## Next

The [Swift SDK page](https://thinkthen.dev/sdks/swift/) shows all ten functions. The [Swift guide](https://github.com/botassembly/thinkthen/blob/v0.2.0/docs/surfaces/swift.md) and the [API reference](https://github.com/botassembly/thinkthen/tree/v0.2.0/sdk/swift/Sources/ThinkThen) cover every option.
