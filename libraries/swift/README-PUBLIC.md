# ThinkThen for Swift

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
tar -xzf thinkthen-swift-0.2.0-TARGET.tar.gz
```

Replace `TARGET` with the platform name on your downloaded release archive.

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

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
        for text in ["Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back."] {
            let refund = try await client.decideAnswer(.text("Does the customer ask for a refund?"), input: .text(text))
            print(refund.map { $0 ? "true" : "false" } ?? "not sure")
        }
        for text in ["Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password."] {
            let team = try await client.chooseAnswer(teamQuestion, input: .text(text))
            print(team ?? "not sure")
        }
    }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
