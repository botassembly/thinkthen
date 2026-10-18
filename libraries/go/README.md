# ThinkThen for Go

ThinkThen is ten functions on top of Jev, TypeSafe's System One model, or any model like it. System One models return probabilities. Your code decides what to do with them. ThinkThen makes these models simple to call from any language or database. Use Jev, OpenAI or a local model. Ask a typed question and get back `true`, `false`, a label or a number. Your code branches on the answer. A model can say not sure, and a failed call never looks like an answer. ThinkThen packs many records into few requests, saves each answer for reuse, and lets tests replay saved answers with no key and no network.

```sh
go get github.com/botassembly/thinkthen/libraries/go@v0.2.0
```
Set your hosted backend key: `export THINKTHEN_API_KEY=...`. Backends include Jev, SystemOne, OpenAI and local models. Judgment calls can send your question and text to the selected backend; the default is TypeSafe.

```go
package main
import ("context"; "fmt"; "log"; tt "github.com/botassembly/thinkthen/libraries/go")
func main() {
    client, err := tt.NewClient(tt.EngineSettings{}); if err != nil { log.Fatal(err) }
    defer client.Close()
    teams := tt.AuthoredOptionsObject{{Key: "billing", Value: tt.AuthoredDescriptionString("Invoices, fees, and refunds.")},
        {Key: "shipping", Value: tt.AuthoredDescriptionString("Parcels and delivery.")},
        {Key: "account", Value: tt.AuthoredDescriptionString("Logins and passwords.")}}
    teamQuestion := tt.DefinedQuestion(tt.RequestDefinitionAuthoredChoose{
        Choose: tt.AuthoredQuestionTextString("Which team owns this?"), Options: tt.Ptr[tt.AuthoredOptions](teams)})
    for i, text := range []string{"Please refund my order. It arrived broken.", "Thanks for the quick help yesterday!", "I want to send this back."} {
        refund, err := client.Decide(context.Background(), tt.TextQuestion("Does the customer ask for a refund?"), tt.TextInput(text), nil)
        if err != nil { log.Fatal(err) }
        value, probability := refund.Value.(bool), float64(refund.Probability.(tt.ProbabilityScalar))
        if value != []bool{true, false, true}[i] || probability != []float64{0.99, 0.01, 0.54}[i] { log.Fatal("unexpected refund answer") }
        fmt.Printf("%t %.2f\n", value, probability)
    }
    for i, text := range []string{"Please refund the extra fee on my invoice.", "My parcel went to the wrong address.", "I cannot reset my password."} {
        team, err := client.Choose(context.Background(), teamQuestion, tt.TextInput(text), nil)
        if err != nil { log.Fatal(err) }
        value, probability := team.Value.(string), float64(team.Probability.(tt.ProbabilityScalar))
        if value != []string{"billing", "shipping", "account"}[i] || probability != []float64{1, 1, 1}[i] { log.Fatal("unexpected team answer") }
        fmt.Printf("%s %.2f\n", value, probability)
    }
}
```

The library is free and MIT licensed. Hosted models charge per token. Local models are free to use; you supply the computer that runs them. These examples were checked with saved answers; live answers and probabilities can differ. See the [package guide](https://thinkthen.dev/languages/go/) for results, failures, record inputs and offline replay.
[Backend setup](https://thinkthen.dev/install/backends/) · [ThinkThen documentation](https://thinkthen.dev)
