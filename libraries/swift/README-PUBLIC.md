# ThinkThen for swift

ThinkThen answers typed questions about text. Use it to decide yes or no, choose a label, score an answer, or filter and rank records. Failed calls remain separate from answers.

## Install

```sh
tar -xzf thinkthen-swift-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded release archive.

## Example

Configure a model backend before running this example.

```swift
import ThinkThen

let client = try Client()
defer { client.close() }
let answer = try await client.decideAnswer(.text("Does this ask for a refund?"),
                                         input: .text("Please refund my order."))
print(answer as Any)
```

## Cost

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
