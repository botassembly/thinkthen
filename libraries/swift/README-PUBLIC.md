# ThinkThen for Swift

ThinkThen puts ten functions on top of any model: decide, choose, tag, score, filter, rank, find, annotate, recognize and relate. Your code branches on typed answers, gets not sure instead of a guess, packs many records into few requests, and caches each question so you pay once. Tests replay saved answers with no key or network. Use Jev, OpenAI or a local model from every language and database.

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
