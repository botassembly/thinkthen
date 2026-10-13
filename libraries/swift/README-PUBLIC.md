# ThinkThen for Swift

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. ThinkThen gives code typed answers to branch on, caches each question so you pay once, and replays recorded answers offline for tests. Backends include Jev, SystemOne, OpenAI and local models.

```sh
tar -xzf thinkthen-swift-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded release archive.

Set your hosted backend key: `export THINKTHEN_API_KEY=...`.

```swift
import ThinkThen

let client = try Client()
defer { client.close() }
let answer = try await client.decideAnswer(.text("Does this ask for a refund?"),
                                         input: .text("Please refund my order."))
print(answer as Any)
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
