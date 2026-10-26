# ThinkThen for Swift

Use Swift 6.3 or later. Add `.package(url: "https://github.com/botassembly/thinkthen", exact: "0.2.0")` to your Swift package dependencies and add the `ThinkThen` product to your target. The package includes its native engine.

The example uses the stub backend and needs no key or service.

```swift
import Foundation
import ThinkThen

let client = try Client(.init(backend: .stub))
let row = try await client.decide("Does the customer ask for money back?",
                                  record: .text("Please refund this purchase."))
if row.answer == true {
    print("Refund requested")
}
```

[API reference](https://github.com/botassembly/thinkthen/tree/v0.2.0/sdk/swift/Sources/ThinkThen).
