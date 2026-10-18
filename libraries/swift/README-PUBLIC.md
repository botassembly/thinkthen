# ThinkThen for Swift

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-swift-0.2.0-TARGET.tar.gz
```

Download the matching archive from the [release downloads](https://github.com/botassembly/thinkthen/releases). Replace `TARGET` with your platform. Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

```swift
import ThinkThen
@main enum RegistryExample {
    static func main() async throws {
        let client = try Client()
        defer { client.close() }
        let teams = OrderedMap<AuthoredDescription>([
            .init("billing", .string("Invoices, fees, and refunds.")),
            .init("shipping", .string("Parcels and delivery.")),
            .init("account", .string("Logins and passwords."))])
        let teamQuestion = Question.definition(QuestionDefinition(value:
            .choose(AuthoredChoose(choose: .string("Which team owns this?"), options: .value(.object(teams))))))
        let refunds = InputRecords(items: ["Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back."].map { .text($0) })
        let refundCall = try await client.decide(.text("Does the customer ask for a refund?"), input: refunds); let refundValues = refundCall.value
        precondition(refundCall.probability == [0.99, 0.01, 0.54])
        for refund in refundValues {
            print(refund.map { $0 == .bool(true) ? "true" : "false" } ?? "not sure")
        }
        let messages = InputRecords(items: ["Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password."].map { .text($0) })
        let teamCall = try await client.choose(teamQuestion, input: messages); let teamValues = teamCall.value
        precondition(teamCall.probability == [1.0, 1.0, 1.0])
        for team in teamValues {
            print(team ?? "not sure")
        }
    }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. [Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev) These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/swift/) for results, failures, record inputs and offline replay.
