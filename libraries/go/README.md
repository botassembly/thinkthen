# ThinkThen for Go

ThinkThen is ten functions on top of Jev, TypeSafe's decision model, or any model like it. It puts these decision models in your code, whatever model, language or database you use. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
go get github.com/botassembly/thinkthen/libraries/go@v0.2.0
```

Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models.

```go
package main
import ("context"; "encoding/json"; "log"; "os"; tt "github.com/botassembly/thinkthen/libraries/go")
func main() {
    client, err := tt.NewClient(tt.EngineSettings{}); if err != nil { log.Fatal(err) }
    defer client.Close()
    teams := tt.AuthoredOptionsObject{{Key: "billing", Value: tt.AuthoredDescriptionString("Invoices, fees, and refunds.")},
        {Key: "shipping", Value: tt.AuthoredDescriptionString("Parcels and delivery.")},
        {Key: "account", Value: tt.AuthoredDescriptionString("Logins and passwords.")}}
    teamQuestion := tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{
        Choose: tt.AuthoredQuestionTextString("Which team owns this?"), Options: tt.Ptr[tt.AuthoredOptions](teams)})
    for _, text := range []string{"Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back."} {
        refund, err := client.Decide(context.Background(), tt.TextQuestion("Does the customer ask for a refund?"), tt.TextInput(text), nil)
        if err != nil { log.Fatal(err) }
        if err := json.NewEncoder(os.Stdout).Encode(refund.Value); err != nil { log.Fatal(err) }
    }
    for _, text := range []string{"Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password."} {
        team, err := client.Choose(context.Background(), teamQuestion, tt.TextInput(text), nil)
        if err != nil { log.Fatal(err) }
        if err := json.NewEncoder(os.Stdout).Encode(team.Value); err != nil { log.Fatal(err) }
    }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them.

[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
